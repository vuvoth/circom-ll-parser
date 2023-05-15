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

use rowan::Children;

type SyntaxNode = rowan::SyntaxNode<SimpleLang>;
#[allow(unused)]
type SyntaxToken = rowan::SyntaxToken<SimpleLang>;
#[allow(unused)]
type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

use crate::token;
use crate::trace::TreeShape;

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
            ADD => (1, 2),
            MUL => (3, 4),
            NUMBER => (10, 0),
            EOF => (10, 10),
            _ => (0, 0),
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    token_kind: Vec<SyntaxKind>,
    token_content: Vec<String>,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<String>,
}

impl Parser {
    fn next(&mut self) -> (SyntaxKind, String) {
        if self.token_kind.is_empty() {
            return (EOF, "".to_string());
        }

        return (self.token_kind.remove(0), self.token_content.remove(0));
    }

    fn peek(&mut self) -> (SyntaxKind, String) {
        if self.token_kind.is_empty() {
            return (EOF, "".to_string());
        }

        return (self.token_kind[0], self.token_content[0].clone());
    }
    fn new(
        token_kind: Vec<SyntaxKind>,
        token_content: Vec<String>,
        builder: GreenNodeBuilder<'static>,
    ) -> Self {
        Parser {
            token_kind,
            token_content,
            builder,
            errors: Vec::<String>::new(),
        }
    }

    pub fn parsing(
        token_kind: Vec<SyntaxKind>,
        token_content: Vec<String>,
        builder: GreenNodeBuilder<'static>,
    ) -> GreenNode {
        let mut p = Parser::new(token_kind, token_content, builder);
        p.builder.start_node(ROOT.into());
        p.parsing_bp(0);

        p.builder.finish_node();
        p.builder.finish()
    }

    pub fn parsing_bp(&mut self, min_bp: u8) -> TreeShape {
        let (token_kind, content) = self.next();
        
        let  mut lhs;

        if token_kind.is_atom() {
             
        } else {
            
        }

        loop {
            let (op, content) = self.peek();

            if op.is_end() {
                break;
            }

            if op.is_atom() {
                panic!("atom can't follow after atom!!!");
            }

            self.builder.start_node(op.into());
            self.builder.token(op.into(), &content);
            // now op is + or *

            let (left_bp, right_bp) = op.power();
            if left_bp < min_bp {
                break;
            }
            self.next();

            self.parsing_bp(right_bp);

        }
    
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
        let token_kind = vec![NUMBER, ADD, NUMBER, MUL, NUMBER];
        let token_content = vec!["10".to_string(), "+".to_string(), "100".to_string(), "*".to_string(), "20".to_string()];
        let builder = GreenNodeBuilder::new();
        let green_node = Parser::parsing(token_kind, token_content, builder);

        let syntax_node = SyntaxNode::new_root(green_node);

        println!("{:?}", syntax_node.to_string());
    
        for child in syntax_node.children() {
            println!("{:?}{:?}", child.kind(), child.text_range());
            // for token in child.children() {
            //     println!("{:?}{:?}", token.kind(), token.text_range());
            // }
        }
    }
}
