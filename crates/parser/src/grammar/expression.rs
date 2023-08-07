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
    // magical
    let Some(mut lhs) = expression_atom(p) else {
        return;
    };

    if p.at(LParen) {
        let m = p.open_before(lhs);
        tuple(p);
        lhs = p.close(m, Call);
    }

    while !p.eof() {
        let current_kind = p.current().kind;
        println!("{:?} + {}\n", current_kind, pb);
        if let Some((lp, rp)) = current_kind.infix() {
            println!("{} {}", rp, pb);
            if rp > pb {
                let m = p.open_before(lhs);
                p.advance();
                expression_rec(p, lp);
                lhs = p.close(m, current_kind);
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{grammar::entry::Scope, token_kind::TokenKind};
    use logos::Lexer;
    #[test]
    fn test_expression() {
        let source = r#"
            hello(a) + hello(b) * 100 + 10 10
        "#;
        let mut lexer = Lexer::<TokenKind>::new(source);
        let mut parser = Parser::new(&mut lexer);

        println!("{}", source);
        expression_rec(&mut parser, 0);

        let cst = parser.build_tree();

        println!("{:?}", cst);
    }
}
