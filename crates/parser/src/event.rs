use crate::{token_kind::TokenKind, node::Token};

#[derive(Debug, Clone, Copy)]
pub enum Event<'a> {
    Open { kind: TokenKind },
    Close,
    Token(Token<'a>),
}
