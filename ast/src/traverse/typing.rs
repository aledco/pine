use crate::ast::*;
use crate::traverse::AstTraverse;

pub struct AstTypeTraverser;

impl AstTraverse for AstTypeTraverser {
    fn traverse(&self, program: &mut Program) {
        let types = program
            .functions
            .iter_mut()
            .map(|f| self.process(f));
    }
}

impl AstTypeTraverser {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    pub fn process(&self, node: &mut AstNode) -> PineType {
        match &mut node.ast_type {
            AstType::Function {
                identifier,
                params,
                return_type_node,
                body
            } => {
                let param_types = params
                    .iter_mut()
                    .map(|p| self.process(p))
                    .collect();
                let return_type = match return_type_node {
                    Some(t) => self.process(t),
                    None => PineType::Void,
                };
                let function_type = PineType::Function {
                    params: param_types,
                    ret: Box::new(return_type)
                };
                node.pine_type = function_type.clone();
                identifier.pine_type = function_type.clone();
                
                self.process(identifier);
                self.process(body);
                
                function_type
            }
            AstType::Param {
                identifier,
                type_node
            } => {
                let param_type = self.process(type_node);
                node.pine_type = param_type.clone();
                identifier.pine_type = param_type.clone();
                self.process(identifier);
                param_type
            }
            AstType::Block(statements) => {
                for s in statements {
                    self.process(s);
                }
                
                node.pine_type = PineType::Void;
                PineType::Void
            }
            AstType::IfStatement {
                condition,
                then_body,
                else_body
            } => {
                let condition_type = self.process(condition);
                // TODO ensure type is boolean

                node.pine_type = PineType::Void;
                PineType::Void
            }
            AstType::WhileStatement {
                condition,
                body
            } => {
                PineType::Void
            }
            AstType::ReturnStatement(expression) => {
                PineType::Void
            }
            AstType::LetStatement {
                identifier,
                type_node,
                expression
            } => {
                PineType::Void
            }
            AstType::SetStatement {
                identifier,
                expression
            } => {
                PineType::Void
            }
            AstType::BinaryExpression {
                lhs,
                op,
                rhs
            } => {
                PineType::Void
            }
            AstType::UnaryExpression {
                op,
                expr
            } => {
                PineType::Void
            }
            AstType::IdentifierExpression(identifier) => {
                PineType::Void
            }
            AstType::IntegerExpression(value) => {
                PineType::Void
            }
            AstType::FloatExpression(value) => {
                PineType::Void
            },
            AstType::BoolExpression(value) => {
                PineType::Void
            }
            AstType::StringExpression(value) => {
                PineType::Void
            }
            AstType::Identifier {
                name,
                symbol
            } => {
                // TODO set symbol type?
                PineType::Void
            }
            AstType::TypeNode(pine_type) => {
                PineType::Void
            }
            AstType::Dummy => {
                PineType::Void
            }
        }
    }
}