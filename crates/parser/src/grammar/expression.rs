use super::*;
pub(super) fn expression(p: &mut Parser) {
    let open_marker = p.open();
    let kind = p.current().kind;
    p.advance();
    p.close(open_marker, kind);
}
