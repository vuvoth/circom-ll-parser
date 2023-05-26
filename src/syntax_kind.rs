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
    TEAMPLATE,
    IDENT,
    UNPARSE,
    LEFT_BR,
    RIGHT_BR,
    BLOCK,
    EOF,
    ROOT, // list of expression,
}

use SyntaxKind::*;

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SimpleLang {}
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



pub type SyntaxNode = rowan::SyntaxNode<SimpleLang>;
#[allow(unused)]
pub type SyntaxToken = rowan::SyntaxToken<SimpleLang>;
#[allow(unused)]
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;


impl SyntaxKind {
    pub fn is_atom(self) -> bool {
        match self {
            NUMBER | IDENT => true,
            _ => false,
        }
    }

    pub fn is_operator(self) -> bool {
        !self.is_atom()
    }

    pub fn is_end(self) -> bool {
        matches!(self, EOF)
    }

    pub fn power(self) -> (u16, u16) {
        match self {
            ADD => (2, 1),
            MUL => (4, 3),
            TEAMPLATE => (6, 5),
            _ => (0, 0)
        }
    }
}

pub struct SyntaxKindStream {
    current_position: usize,
    token_stream: Vec<SyntaxKind>,
}

impl SyntaxKindStream {
    pub fn new(token_stream: Vec<SyntaxKind>) -> SyntaxKindStream {
        Self {
            current_position: 0,
            token_stream,
        }
    }

    pub fn get_pos(&self) -> usize {
        return self.current_position;
    }

    pub fn next(&mut self) -> SyntaxKind {
        let next_token = if self.get_pos() >= self.token_stream.len() {
            EOF
        } else {
            self.token_stream[self.get_pos()]
        };
        self.current_position = self.get_pos() + 1;
        next_token
    }

    pub fn peek(&mut self) -> SyntaxKind {
        let peek_token = if self.get_pos() >= self.token_stream.len() {
            EOF
        } else {
            self.token_stream[self.get_pos()]
        };
        peek_token
    }

    pub fn skip(&mut self) {
        self.next();
    }
}
