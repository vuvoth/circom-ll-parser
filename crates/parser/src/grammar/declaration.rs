use super::*;
pub(super) fn var(p: &mut Parser) {
    let m = p.open();
    p.expect(Var);

    if p.at(LParen) {
        p.skip();
        p.expect(Name);
        while p.at(Comma) && !p.eof(){
            p.expect(Comma);
            p.expect(Name);
        }
        p.expect(RParen);
    } else {
        p.expect(Name);   
        while p.at(Comma) && !p.eof() {
            p.expect(Comma);
            p.expect(Name);
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

    p.expect_any(&[Input, Output]);
    p.expect(Name);
    p.close(m, Signal);
}

