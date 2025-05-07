// mod scoping;
// mod typing;
// 
// use crate::ast::Program;
// use crate::traverse::scoping::AstScopeTraverser;
// use crate::traverse::typing::AstTypeTraverser;
// 
// pub trait AstTraverse {
//     fn traverse(&self, _program: &mut Program) {}
// 
//     fn traverse_mut(&mut self, program: &mut Program) {
//         self.traverse(program)
//     }
// }
// 
// pub fn traverse(program: &mut Program) {
//     let mut traversals: Vec<Box<dyn AstTraverse>> =
//         vec![AstScopeTraverser::new(), AstTypeTraverser::new()];
//     traversals.iter_mut().for_each(|t| t.traverse_mut(program));
// }
// 
// // pub fn process_template(&self, node: &mut AstNode) {
// //     match &mut node.ast_type {
// //         AstType::Function {
// //             identifier,
// //             params,
// //             return_type_node,
// //             body
// //         } => {
// //
// //         }
// //         AstType::Param {
// //             identifier,
// //             type_node
// //         } => {
// //
// //         }
// //         AstType::Block(statements) => {
// //
// //         }
// //         AstType::IfStatement {
// //             condition,
// //             then_body,
// //             else_body
// //         } => {
// //
// //         }
// //         AstType::WhileStatement {
// //             condition,
// //             body
// //         } => {
// //
// //         }
// //         AstType::ReturnStatement(expression) => {
// //
// //         }
// //         AstType::LetStatement {
// //             identifier,
// //             type_node,
// //             expression
// //         } => {
// //
// //         }
// //         AstType::SetStatement {
// //             identifier,
// //             expression
// //         } => {
// //
// //         }
// //         AstType::BinaryExpression {
// //             lhs,
// //             op,
// //             rhs
// //         } => {
// //
// //         }
// //         AstType::UnaryExpression {
// //             op,
// //             expr
// //         } => {
// //
// //         }
// //         AstType::IdentifierExpression(identifier) => {
// //
// //         }
// //         AstType::IntegerExpression(value) => {
// //
// //         }
// //         AstType::FloatExpression(value) => {
// //
// //         }
// //         AstType::StringExpression(value) => {
// //
// //         }
// //         AstType::Identifier {
// //             name,
// //             symbol
// //         } => {
// //
// //         }
// //         AstType::TypeNode(pine_type) => {
// //
// //         }
// //         AstType::Dummy => {}
// //     }
// // }
