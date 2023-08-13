use crate::parser::Parser;
use crate::token_kind::TokenKind::*;


mod block;
mod declaration;
mod expression;
mod include;
mod pragma;
mod template;
mod main_component;
mod list_identity;
mod statement;
/**
 * parse circom program
 */

pub(crate) mod entry {
    use super::*;

    pub fn circom_program(p: &mut Parser) {
        pragma::pragma(p);

        let m = p.open();
        while !p.eof() {
            match p.current().kind {
                Template => template::template(p),
                Include => include::include(p),
                Component => main_component::main_component(p),
                _ => p.advance_with_error("invalid token"),
            }
            p.expect(Comma);
        }
        p.close(m, CircomProgram);
    }

    pub enum Scope {
        Block,
        CircomProgram,
    }

    impl Scope {
        pub fn parse(self, p: &mut Parser) {
            match self {
                Self::Block => block::block(p),
                Self::CircomProgram => circom_program(p),
            }
        }
    }
}
