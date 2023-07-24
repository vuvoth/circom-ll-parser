
use crate::grammar::*;
use super::*;
/**
 * template name() {content}
 *
 */
pub fn template(p: &mut Parser) {
    assert!(p.at(Template));
    let m = p.open();
    p.expect(Template);
    p.expect(Name);
    p.expect(LParen);
    p.expect(RParen);
    block::block(p);
    p.close(m, Template);
}
