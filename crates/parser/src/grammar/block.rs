use super::*;

pub fn block(p: &mut Parser) {
    if !p.at(LCurly) {
        p.advance_with_error("Miss {");
    } else {
        let m = p.open();
        p.eat(LCurly);
        while !p.at(RCurly) && !p.eof() {
            let kind = p.current().kind;
            match kind {
                Signal => declaration::signal(p),
                Var => declaration::var(p),
                _ => expression::expression(p),
            }
            p.expect(Semicolon);
        }

        p.expect(RCurly);

        p.close(m, Block);
    }
}

#[cfg(test)]
mod tests {
    use logos::Lexer;

    use crate::{token_kind::TokenKind, grammar::entry::Scope};

    use super::*;
    #[test]
    fn parse_block_test() {
        let source = r#"
            {
               var x, y; 
               var (x, y);
               var (x, y) = "<expression>";
               var a = x, b = y;
               var a = x, b = y;
               
               signal a; 
               signal a, b;
               signal (a, b);
               signal (a, b) = "<expression>";
            }
        "#;
        let mut lexer = Lexer::<TokenKind>::new(source);
        let mut parser = Parser::new(&mut lexer);

        parser.parse(Scope::Block);

        let cst = parser.build_tree();

        println!("{:?}", cst);   
    }
}