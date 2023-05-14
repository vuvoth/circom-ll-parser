#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
enum SyntaxKind {
    NUMBER = 0,
    MUL,
    ADD,
    WHITESPACE, // whitespaces is explicit
    ERROR,      // as well as errors

    EXPR, // expression
    ROOT, // list of expression
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


#[derive(Debug)]
pub struct Parser {
    tokens: Vec<(SyntaxKind, String)>,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<String>,
}

impl Parser {
    fn new(tokens: Vec<(SyntaxKind, String)>, builder: GreenNodeBuilder<'static>) -> Self {
        Parser {
            tokens,
            builder,
            errors: Vec::<String>::new(),
        }
    }

    fn parsing() -> Self {
        let p = Parser::new(vec![], GreenNodeBuilder::new());
        p
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
}
