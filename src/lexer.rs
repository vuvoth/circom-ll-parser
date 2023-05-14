use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    // Tokens can be literal strings, of any length.
    #[token("+")]
    Add,

    #[token("*")]
    Mul,

    // Or regular expressions.
    #[regex("[0-9]+")]
    Number,
    End
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_tests() {
        let mut lex = Token::lexer("10 + 1 * 2");

        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.span(), 0..2);
        assert_eq!(lex.slice(), "10");
   
        assert_eq!(lex.next(), Some(Ok(Token::Add)));
        assert_eq!(lex.span(), 3..4);
        assert_eq!(lex.slice(), "+");
   
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.span(), 5..6);
        assert_eq!(lex.slice(), "1");
   
        assert_eq!(lex.next(), Some(Ok(Token::Mul)));
        assert_eq!(lex.slice(), "*");
        assert_eq!(lex.span(), 7..8);
   
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.span(), 9..10);
        assert_eq!(lex.slice(), "2");
   
        assert_eq!(lex.next(), None);
    }
}