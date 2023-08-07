use super::{
    expression::{tuple, tuple_init},
    *,
};
pub(super) fn var(p: &mut Parser) {
    let m = p.open();
    p.expect(Var);

    if p.at(LParen) {
        tuple(p);
        if p.at_any(&[Assign, RAssignSignal, RAssignConstraintSignal]) {
            tuple_init(p);
        }
    } else {
        p.expect(Identifier);
        if p.at(Assign) {
            p.expect(Assign);
            expression::expression(p);
        }
        // list of var
        while p.at(Comma) && !p.eof() {
            p.expect(Comma);
            p.expect(Identifier);
            if p.at(Assign) {
                p.expect(Assign);
                expression::expression(p);
            }
        }
    }
    p.close(m, Var);
}

pub(super) fn signal(p: &mut Parser) {
    if !p.at(Signal) {
        p.advance_with_error("Signal error");
        return;
    }

    let m = p.open();
    p.expect(Signal);
    if p.at_any(&[Input, Output]) {
        p.expect_any(&[Input, Output]);
    }

   
    if p.at(LParen) {
        tuple(p);
        if p.at_any(&[Assign, RAssignSignal, RAssignConstraintSignal]) {
            tuple_init(p);
        }
    } else {
        p.expect(Identifier);
        // list of var
        while p.at(Comma) && !p.eof() {
            p.skip();
            p.expect(Identifier);
        }
    } 
    p.close(m, Signal);
}
