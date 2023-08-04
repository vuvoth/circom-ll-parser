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


fn expression_atom(p: &mut Parser) {
    let m = p.open();
    match p.current().kind {
        Number | Identifier => {
            p.advance();
            p.close(m, Number);
        },

        _ => {}
    }
}

fn expression_top(p: &mut Parser) {
    
}