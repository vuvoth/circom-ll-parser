use super::{expression::expression, *, block::block};

pub(super) fn parse(p: &mut Parser) {}

pub(super) fn statement(p: &mut Parser) {
    match p.current().kind {
        IfKw => if_statement(p),
        _ => statement_no_condition(p),
    }
}

fn if_statement(p: &mut Parser) {
    let m = p.open();
    p.expect(IfKw);
    p.expect(LParen);
    expression::expression(p);
    p.expect(RParen);
    statement(p);
    if p.at(ElseKw) {
        p.expect(ElseKw);
        statement(p);
    }
    p.close(m, IfKw);
}

/**
 * no if condition here.
 */
fn statement_no_condition(p: &mut Parser) {
    match p.current().kind {
        ForKw => for_statement(p),
        WhileKw => while_statement(p),
        ReturnKw => {
            return_statement(p);
            p.expect(Semicolon);
        },
        LCurly => block(p),
        _ => {
            assignment_statement(p);
            p.expect(Semicolon);
        },
    }
}

fn for_statement(p: &mut Parser) {
    let m = p.open();
    p.expect(ForKw);
    p.expect(LParen);
    if p.current().kind.is_declaration_kw() {
        declaration::declaration(p);
    } else {
        assignment_statement(p);
    }
    p.expect(Semicolon);
    expression::expression(p);
    p.expect(Semicolon);

    assignment_statement(p);
    p.expect(RParen);

    statement_no_condition(p);
    p.close(m, ForLoop);
}

fn while_statement(p: &mut Parser) {
    p.expect(WhileKw);
    p.expect(LParen);
    expression(p);
    p.expect(RParen);
    statement(p);
}

fn return_statement(p: &mut Parser) {
    let m = p.open();
    p.expect(ReturnKw);
    expression(p);
    p.close(m, ReturnKw);
}


fn assignment_statement(p: &mut Parser) {
    let m = p.open();
    expression(p);
    p.expect_any(&[Assign, RAssignSignal, RAssignConstraintSignal, LAssignContraintSignal, LAssignSignal, EqualSignal]);
    expression(p);
    p.close(m, AssignStatement);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_kind::TokenKind;
    use logos::Lexer;

    #[test]
    fn if_statement_test() {
        let source = r#"
          if (a) {
            a = a + 12;
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
