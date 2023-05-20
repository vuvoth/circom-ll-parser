#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    NUMBER = 0,
    MUL,
    ADD,
    WHITESPACE, // whitespaces is explicit
    ERROR,      // as well as errors

    EXPR, // expression
    ROOT, // list of expression,
    EOF,
}

use SyntaxKind::*;

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum SimpleLang {}
impl rowan::Language for SimpleLang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

use rowan::GreenNode;
use rowan::GreenNodeBuilder;

type SyntaxNode = rowan::SyntaxNode<SimpleLang>;
#[allow(unused)]
type SyntaxToken = rowan::SyntaxToken<SimpleLang>;
#[allow(unused)]
type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

use crate::trace::Trace;

use super::TokenTrait;

impl TokenTrait for SyntaxKind {
    fn end() -> Self {
        SyntaxKind::EOF
    }
    fn is_atom(self) -> bool {
        match self {
            Self::NUMBER => true,
            _ => false,
        }
    }
    fn is_end(self) -> bool {
        matches!(self, Self::EOF)
    }
    fn is_op(self) -> bool {
        !self.is_atom()
    }
    fn power(self) -> (u8, u8) {
        match self {
            ADD => (2, 1),
            MUL => (4, 3),
            NUMBER => (10, 0),
            EOF => (10, 10),
            _ => (0, 0),
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    token_kind: Vec<SyntaxKind>,
    errors: Vec<String>,
    index: u32
}

impl Parser {
    pub fn next(&mut self) -> (SyntaxKind, u32) {
        if self.token_kind.is_empty() {
            return (EOF, self.index);
        }
        self.index = self.index + 1;
        return (self.token_kind.remove(0), self.index - 1);
    }

    pub fn peek(&mut self) -> (SyntaxKind , u32) {
        if self.token_kind.is_empty() {
            return (EOF, self.index);
        }

        return (self.token_kind[0], self.index);
    }
    pub fn new(
        token_kind: Vec<SyntaxKind>,
    ) -> Self {
        Parser {
            token_kind,
            errors: Vec::<String>::new(),
            index: 0
        }
    }

    pub fn parsing(
        token_kind: Vec<SyntaxKind>,
        token_content: &Vec<String>,
    ) -> GreenNode {
        let mut p = Parser::new(token_kind);
        let tree = p.parsing_bp(0);
        println!("{:#?}", tree);
        
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(ROOT.into());
        tree.build(&mut builder, token_content);
        builder.finish_node();
        builder.finish()
    }

    pub fn parsing_bp(&mut self, min_bp: u8) -> Trace {
        let (token_kind, id) = self.next();
        
        println!("{}", id);
        let mut lhs_node = if token_kind.is_atom() {
            Trace::new(token_kind,Some(id), vec![])
        } else {
            let op = token_kind;
            let (_left_bp, right_bp) = op.power();

            let mut tree = Trace::new(token_kind, None, vec![]);

            let op_node = Trace::new(token_kind, Some(id), vec![]);
            let right_node = self.parsing_bp(right_bp);

            tree.add_child(op_node);
            tree.add_child(right_node);
            tree
        };

        loop {
            let (op, op_id) = self.peek();

            println!("loop: {}", op_id);
        
            if op.is_end() {
                break;
            }

            if op.is_atom() {
                panic!("atom can't follow after atom!!!");
            }

            let mut tree = Trace::new(op, None, vec![]);
            let op_node = Trace::new(op, Some(op_id), vec![]);

            // now op is + or *

            let (left_bp, right_bp) = op.power();
            if left_bp < min_bp {
                break;
            }

            self.next();

            let rhs_node = self.parsing_bp(right_bp);

            // union tree 
            tree.add_child(lhs_node);
            tree.add_child(op_node);
            tree.add_child(rhs_node);

            lhs_node = tree
        }
        lhs_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rowan() {
        let mut builder = GreenNodeBuilder::new();

        builder.start_node(ROOT.into());
        builder.start_node(ADD.into());
        builder.token(ADD.into(), "+");

        builder.start_node(NUMBER.into());
        builder.token(NUMBER.into(), "10");
        builder.finish_node();

        builder.start_node(NUMBER.into());
        builder.token(NUMBER.into(), "10");
        builder.finish_node();
        builder.finish_node();

        builder.start_node(ADD.into());
        builder.token(ADD.into(), "+");
        builder.start_node(NUMBER.into());
        builder.token(NUMBER.into(), "3");
        builder.finish_node();
        builder.start_node(NUMBER.into());
        builder.token(NUMBER.into(), "100");
        builder.finish_node();
        builder.finish_node();

        builder.finish_node();

        let green_node = builder.finish();
        eprintln!("{:?}", green_node);
        let syntax_node = SyntaxNode::new_root(green_node.clone());

        eprintln!("{:?}", syntax_node);
        for child in syntax_node.children() {
            println!("{:?}{:?}", child.kind(), child.text_range());
            for token in child.children() {
                println!("{:?}{:?}", token.kind(), token.text_range());
            }
        }
    }

    #[test]
    fn test_parser_build_tree() {
        let token_kind = vec![NUMBER, MUL, NUMBER, ADD, NUMBER];
        let token_content = vec!["10".to_string(), "*".to_string(), "100".to_string(), "+".to_string(), "20".to_string()];
        let green_node = Parser::parsing(token_kind, &token_content);

        let syntax_node = SyntaxNode::new_root(green_node);
        println!("{}", syntax_node); 
    }
}
