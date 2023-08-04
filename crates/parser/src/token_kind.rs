use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex("[ \t]+", logos::skip)]
    #[regex("[\n]+", logos::skip)]
    #[error]
    Error = 0,

    #[token("pragma")]
    Pragma,
    #[token("circom")]
    Circom,
    #[regex("2.[0-9].[0-9]")]
    Version,
    #[regex("[0-9]+")]
    Number,
    #[regex("[$_]*[a-zA-Z][a-zA-Z0-9_$]*")]
    Identifier,
    #[regex(r#""[^"]*""#)]
    String,
    #[token("template")]
    Template,
    #[token("component")]
    Component,
    #[token("main")]
    Main,
    #[token("public")]
    Public,
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
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token("=")]
    Assign,
    #[token("-->")]
    LAssignSignal, 
    #[token("==>")]
    LAssignContraintSignal,
    #[token("<--")]
    RAssignSignal,
    #[token("<==")]
    RAssignConstraintSignal,
    CircomProgram,
    Block,
    Tuple,
    TupleInit,
    EOF,
}
