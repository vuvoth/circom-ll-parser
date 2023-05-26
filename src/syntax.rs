



// use SyntaxKind::*;
// use crate::grammar;
// use crate::syntax_kind::SyntaxKind;


// use rowan::GreenNode;
// use rowan::GreenNodeBuilder;

// type SyntaxNode = rowan::SyntaxNode<SimpleLang>;
// #[allow(unused)]
// type SyntaxToken = rowan::SyntaxToken<SimpleLang>;
// #[allow(unused)]
// type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

// use crate::trace::Output;
// // 
// use super::TokenTrait;

// impl TokenTrait for SyntaxKind {
//     fn end() -> Self {
//         SyntaxKind::EOF
//     }
//     fn is_atom(self) -> bool {
//         match self {
//             Self::NUMBER => true,
//             _ => false,
//         }
//     }
//     fn is_end(self) -> bool {
//         matches!(self, Self::EOF)
//     }
//     fn is_op(self) -> bool {
//         !self.is_atom()
//     }
//     fn power(self) -> (u8, u8) {
//         match self {
//             ADD => (2, 1),
//             MUL => (4, 3),
//             NUMBER => (10, 0),
//             EOF => (10, 10),
//             TEAMPLATE => (7, 8),
//             LEFT_BR => (6, 7),
//             RIGHT_BR => (6, 7),
//             IDENT => (10, 0),
//             _ => (0, 0),
//         }
//     }
// }

// #[derive(Debug)]
// pub struct Parser {
//     token_kind: Vec<SyntaxKind>,
//     errors: Vec<String>,
//     index: u32,
// }

// impl Parser {
//     pub fn next(&mut self) -> (SyntaxKind, u32) {
//         if self.token_kind.is_empty() {
//             return (EOF, self.index);
//         }
//         self.index = self.index + 1;
//         return (self.token_kind.remove(0), self.index - 1);
//     }

//     pub fn peek(&mut self) -> (SyntaxKind, u32) {
//         if self.token_kind.is_empty() {
//             return (EOF, self.index);
//         }

//         return (self.token_kind[0], self.index);
//     }
//     pub fn new(token_kind: Vec<SyntaxKind>) -> Self {
//         Parser {
//             token_kind,
//             errors: Vec::<String>::new(),
//             index: 0,
//         }
//     }

//     pub fn parsing(token_kind: Vec<SyntaxKind>, token_content: &Vec<String>) -> GreenNode {
//         let mut p = Parser::new(token_kind);
//         let tree = p.parsing_bp(0);

//         let mut builder = GreenNodeBuilder::new();
//         builder.start_node(ROOT.into());
//         tree.construct(&mut builder, token_content);
//         builder.finish_node();
//         builder.finish()
//     }

//     pub fn parsing_block_out(token_kind: Vec<SyntaxKind>, token_content: &Vec<String>) -> GreenNode {
//         let mut p = Parser::new(token_kind);
//         let tree = p.parsing_block(0);

//         let mut builder = GreenNodeBuilder::new();
//         builder.start_node(ROOT.into());
//         tree.construct(&mut builder, token_content);
//         builder.finish_node();
//         builder.finish()
//     }


//     pub fn parsing_block(&mut self, min_bp: u8) -> Output {
//         let open = self.next();
//         let mut tree = Output::new(BLOCK, None, vec![]);
//         let open_node: Output = Output::new(LEFT_BR, Some(open.1), vec![]);
//         tree.add_child(open_node);

//         loop {
//             let op = self.peek();
//             if op.0.is_end() {
//                 break;
//             }

//             let block_node = if matches!(op.0, RIGHT_BR) {
//                Output::new(RIGHT_BR, Some(op.1), vec![])
//             } else {
//                 self.parsing_bp(min_bp)
//             };

//             tree.add_child(block_node);
//         }
//         tree
//     }

//     pub fn parsing_bp(&mut self, min_bp: u8) -> Output {
//         let (token_kind, id) = self.next();

//         let mut lhs_node = if token_kind.is_atom() {
//             Output::new(token_kind, Some(id), vec![])
//         } else {
//             let op = token_kind;
//             let (_left_bp, right_bp) = op.power();

//             let mut tree = Output::new(token_kind, None, vec![]);

//             let op_node = Output::new(token_kind, Some(id), vec![]);
//             let right_node = self.parsing_bp(right_bp);

//             tree.add_child(op_node);
//             tree.add_child(right_node);
//             tree
//         };

//         loop {
//             let (op, op_id) = self.peek();

//             if op.is_end() {
//                 break;
//             }

//             if op.is_atom() {
//                 panic!("atom can't follow after atom!!!");
//             }

//             let mut tree = Output::new(op, None, vec![]);
//             let op_node = Output::new(op, Some(op_id), vec![]);

//             // now op is + or * or template

//             let (left_bp, right_bp) = op.power();
//             if left_bp < min_bp {
//                 break;
//             }


//             if matches!(op, TEAMPLATE) {
//                 self.next();
//                 let (_, name_id) = self.peek();
//                 let name_node = Output::new(IDENT, Some(name_id), vec![]);
//                 self.next();

//                 let rhs_node = self.parsing_bp(right_bp);
//                 tree.add_child(op_node);
//                 tree.add_child(name_node);
//                 tree.add_child(rhs_node);
//             } else if matches!(op, LEFT_BR) {
//                 let (_, name_id) = self.peek();
//                 return self.parsing_block(min_bp);
//             } 
//             else {
//                 self.next();
//                 let rhs_node = self.parsing_bp(right_bp);

//                 // union tree
//                 tree.add_child(lhs_node);
//                 tree.add_child(op_node);
//                 tree.add_child(rhs_node);
//             }

//             lhs_node = tree
//         }
//         lhs_node
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_rowan() {
//         let mut builder = GreenNodeBuilder::new();

//         builder.start_node(ROOT.into());
//         builder.start_node(ADD.into());
//         builder.token(ADD.into(), "+");

//         builder.start_node(NUMBER.into());
//         builder.token(NUMBER.into(), "10");
//         builder.finish_node();

//         builder.start_node(NUMBER.into());
//         builder.token(NUMBER.into(), "10");
//         builder.finish_node();
//         builder.finish_node();

//         builder.start_node(ADD.into());
//         builder.token(ADD.into(), "+");
//         builder.start_node(NUMBER.into());
//         builder.token(NUMBER.into(), "3");
//         builder.finish_node();
//         builder.start_node(NUMBER.into());
//         builder.token(NUMBER.into(), "100");
//         builder.finish_node();
//         builder.finish_node();

//         builder.finish_node();

//         let green_node = builder.finish();
//         eprintln!("{:?}", green_node);
//         let syntax_node = SyntaxNode::new_root(green_node.clone());

//         eprintln!("{:?}", syntax_node);
//         for child in syntax_node.children() {
//             println!("{:?}{:?}", child.kind(), child.text_range());
//             for token in child.children() {
//                 println!("{:?}{:?}", token.kind(), token.text_range());
//             }
//         }
//     }

//     #[test]
//     fn test_parser_build_tree() {
//         let token_kind = vec![NUMBER, MUL, NUMBER, ADD, NUMBER];
//         let token_content = vec![
//             "10".to_string(),
//             "*".to_string(),
//             "100".to_string(),
//             "+".to_string(),
//             "20".to_string(),
//         ];
//         let green_node = Parser::parsing(token_kind, &token_content);

//         let syntax_node = SyntaxNode::new_root(green_node);
//         println!("{:?}", syntax_node.kind());
//     }
//     #[test]
//     fn test_parser_template() {
//         let token_kind = vec![TEAMPLATE, IDENT, NUMBER, ADD, NUMBER];
//         let token_content = vec![
//             "template".to_string(),
//             "name".to_string(),
//             "10".to_string(),
//             "+".to_string(),
//             "20".to_string(),
//         ];
//         let green_node = Parser::parsing(token_kind, &token_content);

//         let syntax_node = SyntaxNode::new_root(green_node).last_child().unwrap();
//         println!("{:?}", syntax_node);
//         for child in syntax_node.children() {
//             println!("{:?}", child);
//         }
//     }

//     #[test] 
//     fn test_parser_block() {
//         let token_kind = vec![LEFT_BR, NUMBER, ADD, NUMBER, RIGHT_BR];
//         let token_content = vec![
//             "{".to_string(),
//             "1".to_string(),
//             "+".to_string(),
//             "3".to_string(),
//             "}".to_string()
//         ];

//         let green_node = Parser::parsing_block_out(token_kind, &token_content);
//         let syntax_node = SyntaxNode::new_root(green_node).last_child().unwrap();
//         println!("{:?}", syntax_node);
//         for child in syntax_node.children() {
//             println!("{:?}", child);
//         }

//     }
// }
