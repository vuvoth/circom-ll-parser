use super::*;


pub fn main_component(p: &mut Parser) {
    p.expect(Component);
    p.expect(Main);
    p.expect(LCurly);
    p.expect(Public);
    p.expect(LBracket);
    list_identity::parse(p);
    p.expect(RBracket);
    p.expect(Assign);
    expression::expression(p);
}