use super::*;

pub(super) fn include(p: &mut Parser) {
    assert!(p.at(Include));

    let m = p.open();
    p.expect(Include);
    p.expect(String);
    p.expect(Semicolon);
    p.close(m, Include);
}