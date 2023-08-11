use super::*;

pub(super) fn parse(p: &mut Parser) {}

fn if_statement(p: &mut Parser) {
    let m = p.open();
    p.expect(IfKw);
    p.expect(LParen);
    expression::expression(p);
    p.expect(RParen);
    p.expect(LCurly);
    expression::expression(p);
    p.expect(RCurly);
    p.close(m, IfKw);
}

/**
 * no if condition here.
 */
pub(super) fn statement_no_condition(p: &mut Parser) {
    match p.current().kind {
        ForKw => for_statement(p),
        _ => unreachable!(),
    }
}

fn for_statement(p: &mut Parser) {
    let m = p.open();
    p.expect(ForKw);
    p.expect(LParen);
    if p.current().kind.is_declaration_kw() {
        declaration::declaration(p);
    } else {
        assign_statement(p);
    }
    p.expect(Semicolon);
    expression::expression(p);
    p.expect(Semicolon);

    assign_statement(p);
    p.expect(RParen);

    statement_no_condition(p);
    p.close(m, ForLoop);
}

fn while_statement() {
    
}

fn return_statement() {}

fn assign_statement(p: &mut Parser) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_kind::TokenKind;
    use logos::Lexer;

    #[test]
    fn if_statement_test() {
        let source = r#"
          if (a) {
            a + b
          }
        "#;
        let mut lexer = Lexer::<TokenKind>::new(source);
        let mut parser = Parser::new(&mut lexer);

        println!("{}", source);
        if_statement(&mut parser);
        let cst = parser.build_tree();

        println!("{:?}", cst);
    }
}
