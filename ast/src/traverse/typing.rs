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
                let e_type = self.process(expression);
                let i_type = self.process(identifier);
                if i_type != e_type {
                    panic!("Type Error at {}: types do not match", node.span);
                }

                node.pine_type = PineType::Void;
                PineType::Void
            }
            AstType::BinaryExpression {
                lhs,
                op,
                rhs
            } => {
                let lhs_type = self.process(lhs);
                let rhs_type = self.process(rhs);
                if lhs_type != rhs_type {
                    panic!("Type Error at {}: types do not match", node.span);
                }
                
                // TODO ensure operator is defined for this type

                lhs_type.clone()
            }
            AstType::UnaryExpression {
                op,
                expr
            } => {
                let expr_type = self.process(expr);
                // TODO ensure operator is defined for this type
                expr_type
            }
            AstType::IdentifierExpression(identifier) => {
                self.process(identifier)
            }
            AstType::IntegerExpression(value) => {
                node.pine_type = PineType::Integer;
                PineType::Integer
            }
            AstType::FloatExpression(value) => {
                node.pine_type = PineType::Float;
                PineType::Float
            },
            AstType::BoolExpression(value) => {
                node.pine_type = PineType::Bool;
                PineType::Bool
            }
            AstType::StringExpression(_) => {
                node.pine_type = PineType::String;
                PineType::String
            }
            AstType::Identifier {
                symbol,
                ..
            } => {
                // TODO set symbol type, if identifier does not have a type set, set it from symbol
                if node.pine_type != PineType::Unknown {
                    symbol.borrow_mut().pine_type = node.pine_type.clone();
                } else if symbol.borrow().pine_type != PineType::Unknown {
                    node.pine_type = symbol.borrow().pine_type.clone();
                } else {
                    panic!("Type Error at {}: Type is unknown", node.span);
                }

                node.pine_type.clone()
            }
            AstType::TypeNode(pine_type) => {
                node.pine_type = pine_type.clone();
                node.pine_type.clone()
            }
            AstType::Dummy => {
                PineType::Void
            }
        }
    }
}