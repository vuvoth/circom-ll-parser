use logos::Logos;

#[derive(Logos, Debug, PartialEq,Clone, Copy)]
pub enum TokenKind {
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex("[ \t]+", logos::skip)]
    #[regex("[\n]+", logos::skip)]
    #[error]
    Error = 0,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Name,

    #[token("template")]
    Template,
    #[token("signal")]
    Signal,
    #[token("input")]
    Input,
    #[token("output")]
    Output,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LCurly,
    #[token("}")]
    RCurly,
    File,
    Block,
    EOF,
}
