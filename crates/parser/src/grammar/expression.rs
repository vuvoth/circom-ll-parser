use crate::{node::Token, parser::Marker};

use super::*;
pub(super) fn expression(p: &mut Parser) {
    let open_marker = p.open();
    let kind = p.current().kind;
    p.advance();
    p.close(open_marker, kind);
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
        Number | Identifier => {
            p.advance();
            m_close = p.close(m, Number);
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

pub(super) fn expression_rec(p: &mut Parser, pb: u16) {
    println!("{:?} {}", p.current(), pb);
    // magical
    let Some(mut lhs) = expression_atom(p) else {
        return;
    };

    while !p.eof() {
        println!("  {:?}", p.current());
        let current_kind = p.current().kind;
        if let Some((lp, rp)) = current_kind.infix() {
            if rp > pb {
                let m = p.open_before(lhs);
                p.advance();
                expression_rec(p, lp);
                lhs = p.close(m, current_kind);
            } else {
                break;
            }
        }
        break;
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
             4 + (12 + 3) + 10 * 9
        "#;
        let mut lexer = Lexer::<TokenKind>::new(source);
        let mut parser = Parser::new(&mut lexer);

        expression_rec(&mut parser, 0);

        let cst = parser.build_tree();

        println!("{:?}", cst);
    }
}
