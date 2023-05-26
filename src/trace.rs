use rowan::{GreenNode, GreenNodeBuilder};

use crate::syntax_kind::SyntaxKind;

#[derive(Debug)]
pub struct Output {
    token_kind: SyntaxKind,
    token_id: Option<usize>,
    children: Vec<Output>,
}

impl Output {
    pub fn new(token_kind: SyntaxKind, token_id: Option<usize>) -> Self {
        return Output {
            token_kind,
            token_id,
            children: vec![],
        };
    }

    pub fn build_green_node(
        self,
        node_kind: SyntaxKind,
        token_contents: &Vec<String>,
    ) -> GreenNode {
        let mut builder = GreenNodeBuilder::new();
        
        builder.start_node(node_kind.into());
        self.construct(&mut builder, token_contents);
        builder.finish_node();


        builder.finish()
    }

    pub fn construct(self, builder: &mut GreenNodeBuilder<'static>, tokens: &Vec<String>) {
        builder.start_node(self.token_kind.into());
        if let Some(id) = self.token_id {
            builder.token(self.token_kind.into(), &tokens[id]);
        }

        for child in self.children {
            child.construct(builder, tokens);
        }

        builder.finish_node();
    }

    pub fn add_child(&mut self, child: Output) -> bool {
        self.children.push(child);
        true
    }
}
