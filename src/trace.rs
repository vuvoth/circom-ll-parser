use rowan::{GreenNode, GreenNodeBuilder};

use crate::syntax::SyntaxKind;

#[derive(Debug)]
pub struct TreeShape {
    token_kind: SyntaxKind,
    token_content: Option<String>,
    children: Vec<TreeShape>,
}

impl TreeShape {
    pub fn new(
        token_kind: SyntaxKind,
        token_content: Option<String>,
        children: Vec<TreeShape>,
    ) -> Self {
        return TreeShape {
            token_kind,
            token_content,
            children,
        };
    }

    pub fn build(self, builder: &mut GreenNodeBuilder<'static>) {
        builder.start_node(self.token_kind.into());
        if let Some(content) = self.token_content {
            builder.token(self.token_kind.into(), &content);
        }

        for child in self.children {
            child.build(builder);
        }
        
        builder.finish_node();
    }

    pub fn add_child(&mut self, child: TreeShape) -> bool {
        self.children.push(child);
        true
    }
}
