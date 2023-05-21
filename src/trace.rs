use rowan::{GreenNode, GreenNodeBuilder};

use crate::syntax::SyntaxKind;

#[derive(Debug)]
pub struct Trace {
    token_kind: SyntaxKind,
    token_id: Option<u32>,
    children: Vec<Trace>,
}

impl Trace {
    pub fn new(
        token_kind: SyntaxKind,
        is_token_node: Option<u32>,
        children: Vec<Trace>,
    ) -> Self {
        return Trace {
            token_kind,
            token_id: is_token_node,
            children,
        };
    }

    pub fn build(self, builder: &mut GreenNodeBuilder<'static>, tokens: &Vec<String>) {
        builder.start_node(self.token_kind.into());
        if let Some(id) = self.token_id {
            builder.token(self.token_kind.into(), &tokens[id as usize]);
        }

        for child in self.children {
            child.build(builder, tokens);
        }
        
        builder.finish_node();
    }

    pub fn add_child(&mut self, child: Trace) -> bool {
        self.children.push(child);
        true
    }
}
