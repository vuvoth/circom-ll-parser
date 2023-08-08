use crate::{node::Token, parser::Marker};

use super::*;
pub(super) fn expression(p: &mut Parser) {
    expression_rec(p, 0);
}

/**
 * grammar: "(Symbol_1, Symbol_2,..., Symbol_n)"
 */
pub(super) fn tuple(p: &mut Parser) {
    let m = p.open();
    p.expect(LParen);
    p.expect(Identifier);
    while p.at(Comma) && !p.eof() {
        p.expect(Comma);
        p.expect(Identifier);
    }
    p.expect(RParen);
    p.close(m, Tuple);
}

/**
 * grammar:
 *      "= | <== | <--" expression
 */
pub(super) fn tuple_init(p: &mut Parser) {
    let m = p.open();
    p.expect_any(&[Assign, RAssignSignal, RAssignConstraintSignal]);
    expression(p);
    p.close(m, TupleInit);
}

fn expression_atom(p: &mut Parser) -> Option<Marker> {
    let m = p.open();
    let m_close: Marker;
    match p.current().kind {
        Number => {
            p.advance();
            m_close = p.close(m, Number);
            return Some(m_close);
        }
        Identifier => {
            p.advance();
            m_close = p.close(m, Identifier);
            return Some(m_close);
        }
        LParen => {
            p.expect(LParen);
            expression_rec(p, 0);
            p.expect(RParen);
            m_close = p.close(m, Tuple);
            return Some(m_close);
        }
        _ => return None,
    }
}

pub fn expression_rec(p: &mut Parser, pb: u16) {
    let parse_able: Option<Marker> = if let Some(pp) = p.current().kind.prefix() {
        let kind = p.current().kind;
        let m = p.open();
        p.advance();
        expression_rec(p, pp);
        Some(p.close(m, kind))
    } else {
        expression_atom(p)
    };

    if parse_able.is_none() {
        return;
    }

    let mut lhs = parse_able.unwrap();

    // TODO: function call
    if p.at(LParen) {
        let m = p.open_before(lhs);
        tuple(p);
        lhs = p.close(m, Call);
    }

    while !p.eof() {
        let current_kind = p.current().kind;
        if let Some((lp, rp)) = current_kind.infix() {
            if !(rp > pb) {
                return;
            }

            let m = p.open_before(lhs);
            p.advance();
            expression_rec(p, lp);
            lhs = p.close(m, current_kind);

            continue;
        }

        if matches!(current_kind, MarkQuestion) {
            let m = p.open_before(lhs);
            lhs = p.close(m, Condition);
            let m = p.open_before(lhs);
            p.advance();
            let first_expression = p.open();
            expression_rec(p, 0);
            p.close(first_expression, Expression);
            p.expect(Colon);
            let last_expression = p.open();
            expression_rec(p, 0);
            p.close(last_expression, Expression);
            lhs = p.close(m, TenaryConditional);
            continue;
        }

        break;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_kind::TokenKind;
    use logos::Lexer;
    #[test]
    fn test_expression() {
        let source = r#"
          a ? a + 10 * 100 : b(x) - 20
        "#;
        let mut lexer = Lexer::<TokenKind>::new(source);
        let mut parser = Parser::new(&mut lexer);

        println!("{}", source);
        expression_rec(&mut parser, 0);
        // println!("{:?}", parser.events);
        let cst = parser.build_tree();

        println!("{:?}", cst);
    }
}
