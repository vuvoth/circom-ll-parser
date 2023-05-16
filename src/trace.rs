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
        for child in self.children {
            println!("{:?}| str: {}", child, self.token_content.as_deref().unwrap_or("abs").to_string());
            if child.token_content.is_some() {
                builder.token(
                    child.token_kind.into(),
                    &child.token_content.as_deref().unwrap().to_string(),
                );
            } else {
                child.build(builder);
            }
        }
        builder.finish_node();
    }

    pub fn add_child(&mut self, child: TreeShape) -> bool {
        self.children.push(child);
        true
    }
}
