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

                // TODO ensure return type is valid
                // TODO ensure all paths return if return type is not void

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
                if condition_type != PineType::Bool {
                    panic!("Type Error at {}: condition must have type bool", node.span);
                }

                self.process(then_body);
                if let Some(else_body) = else_body {
                    self.process(else_body);
                }

                node.pine_type = PineType::Void;
                PineType::Void
            }
            AstType::WhileStatement {
                condition,
                body
            } => {
                let condition_type = self.process(condition);
                if condition_type != PineType::Bool {
                    panic!("Type Error at {}: condition must have type bool", node.span);
                }

                self.process(body);

                node.pine_type = PineType::Void;
                PineType::Void
            }
            AstType::ReturnStatement(expression) => {
                if let Some(expression) = expression {
                    self.process(expression);
                }

                node.pine_type = PineType::Void;
                PineType::Void
            }
            AstType::LetStatement {
                identifier,
                type_node,
                expression
            } => {
                let e_type = self.process(expression);
                if let Some(type_node) = type_node {
                    let n_type = self.process(type_node);
                    if n_type != e_type {
                        panic!("Type Error at {}: types do not match", node.span);
                    }
                }

                identifier.pine_type = e_type.clone();
                self.process(identifier);

                node.pine_type = PineType::Void;
                PineType::Void
            }
            AstType::SetStatement {
                identifier,
                expression
            } => {
                // TODO finish

                node.pine_type = PineType::Void;
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