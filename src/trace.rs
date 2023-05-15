use rowan::{GreenNode, GreenNodeBuilder};

use crate::syntax::SyntaxKind;

pub struct TreeShape<'a> {
    token_kind: SyntaxKind,
    token_content: Option<&'a str>,
    children: Vec<TreeShape<'a>>,
}

impl TreeShape<'_> {
    pub fn new(
        token_kind: SyntaxKind,
        token_content: Option<&str>,
        children: Vec<TreeShape>,
    ) -> Self {
        return TreeShape { token_kind, token_content, children };
    }

    pub fn build(self, builder: &mut GreenNodeBuilder<'static>, tokens: &Vec<String>) {
        builder.start_node(self.token_kind.into());
        for child in self.children {
            if let Some(content) = self.token_content{
                builder.token(child.token_kind.into(), content);
            } else {
                child.build(builder, tokens);
            }
        }
        builder.finish_node();
    }
}
