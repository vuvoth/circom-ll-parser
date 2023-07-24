use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex("[ \t]+", logos::skip)]
    #[regex("[\n]+", logos::skip)]
    #[error]
    Error = 0,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Name,

    #[regex(r#""[^"]*""#)]
    String,
    #[token("pragma")]
    Pragma,
    #[token("circom")]
    Circom,
    #[regex("2.[0-9].[0-9]")]
    Version,
    #[token("template")]
    Template,
    #[token("signal")]
    Signal,
    #[token("var")]
    Var,
    #[token("include")]
    Include,
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
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    CircomProgram,
    Block,
    EOF,
}
