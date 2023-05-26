use crate::syntax_kind::*;
use crate::{syntax_kind::SyntaxKindStream, trace::Output};

pub fn expression_bp(syntax_kind_stream: &mut SyntaxKindStream, min_bp: u16) -> Output {
    let pos = syntax_kind_stream.get_pos();
    let token_kind = syntax_kind_stream.next();

    let mut lhs_node = if token_kind.is_atom() {
        Output::new(token_kind, Some(pos))
    } else {
        let op = token_kind;
        let (_left_bp, right_bp) = op.power();

        let mut tree = Output::new(token_kind, None);

        let op_node = Output::new(token_kind, Some(pos));
        let right_node = expression_bp(syntax_kind_stream, right_bp);

        tree.add_child(op_node);
        tree.add_child(right_node);
        tree
    };

    loop {
        let op = syntax_kind_stream.peek();
        let pos_id = syntax_kind_stream.get_pos();

        if op.is_end() {
            break;
        }

        if op.is_atom() {
            panic!("atom can't follow after atom!!!");
        }

        let mut tree = Output::new(op, None);
        let op_node = Output::new(op, Some(pos_id));

        // now op is + or * or template

        let (left_bp, right_bp) = op.power();
        if left_bp < min_bp {
            break;
        }
        syntax_kind_stream.next();
        let rhs_node = expression_bp(syntax_kind_stream, right_bp);

        // union tree
        tree.add_child(lhs_node);
        tree.add_child(op_node);
        tree.add_child(rhs_node);

        lhs_node = tree
    }
    lhs_node
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::syntax_kind::SyntaxKind::*;
    use crate::syntax_kind::SyntaxNode;

    #[test]
    fn expression_bp_test() {
        let token_kinds = vec![NUMBER, MUL, NUMBER, ADD, NUMBER];
        let token_contents = vec!["10", "+", "100", "*", "20"]
            .iter()
            .map(|v| v.to_string())
            .collect();

        let mut syntax_kind_stream = SyntaxKindStream::new(token_kinds);
        let parse_output = expression_bp(&mut syntax_kind_stream, 0);

        let green_node = parse_output.build_green_node(ROOT, &token_contents);

        let syntax_node = SyntaxNode::new_root(green_node).last_child().unwrap();

        for child in syntax_node.children() {
            println!("{:?}", child);
            for c in child.children() {
                println!("{:?}", c);
            }
        }
        println!("{:?}", syntax_node.to_string());
    }
}
