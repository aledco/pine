use crate::ast::*;
use crate::traverse::Traverse;

pub struct AstTypeTraverser;

impl Traverse for AstTypeTraverser {
    fn traverse(&mut self, program: &mut Program) {
        for f in &mut program.functions {
            self.visit_function(f);
        }
    }
}

impl AstTypeTraverser {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    fn visit_function(&self, function: &mut Function) -> PineType {
        let param_types = function
            .params
            .iter_mut()
            .map(|p| self.visit_param(p))
            .collect();

        let return_type = match &mut function.return_type_node {
            Some(t) => self.visit_type_node(t.as_mut()),
            None => PineType::Void,
        };

        let function_type = PineType::Function {
            params: param_types,
            ret: Box::new(return_type),
        };

        function.identifier.set_pine_type(function_type.clone());

        self.visit_identifier(function.identifier.as_mut());
        self.visit_statement(function.body.as_mut());

        // TODO ensure return type is valid
        // TODO ensure all paths return if return type is not void

        function_type
    }

    fn visit_param(&self, param: &mut Param) -> PineType {
        let param_type = self.visit_type_node(param.type_node.as_mut());
        param.identifier.set_pine_type(param_type.clone());
        self.visit_identifier(param.identifier.as_mut());
        param_type
    }

    fn visit_statement(&self, statement: &mut Statement) -> PineType {
        match &mut statement.statement_type {
            StatementType::Let(identifier, type_node, expression) => {
                let e_type = self.visit_expression(expression);
                if let Some(type_node) = type_node {
                    let n_type = self.visit_type_node(type_node);
                    if n_type != e_type {
                        panic!("Type Error at {}: types do not match", statement.span());
                    }

                    identifier.set_pine_type(n_type.clone());
                } else {
                    identifier.set_pine_type(e_type.clone());
                }

                self.visit_identifier(identifier);
            }
            StatementType::Set(identifier, expression) => {
                let e_type = self.visit_expression(expression);
                let i_type = self.visit_identifier(identifier);
                if i_type != e_type {
                    panic!("Type Error at {}: types do not match", statement.span());
                }
            }
            StatementType::If(condition, if_body, else_body) => {
                let condition_type = self.visit_expression(condition);
                if condition_type != PineType::Bool {
                    panic!(
                        "Type Error at {}: condition must have type bool",
                        statement.span()
                    );
                }

                self.visit_statement(if_body);
                if let Some(else_body) = else_body {
                    self.visit_statement(else_body);
                }
            }
            StatementType::While(condition, body) => {
                let condition_type = self.visit_expression(condition);
                if condition_type != PineType::Bool {
                    panic!(
                        "Type Error at {}: condition must have type bool",
                        statement.span()
                    );
                }

                self.visit_statement(body);
            }
            StatementType::Return(expression) => {
                if let Some(expression) = expression {
                    self.visit_expression(expression);
                }
            }
            StatementType::Expression(expression) => {
                self.visit_expression(expression);
            }
            StatementType::Block(statements) => {
                for s in statements {
                    self.visit_statement(s);
                }
            }
        }

        PineType::Void // statements always have type void
    }

    fn visit_expression(&self, expression: &mut Expression) -> PineType {
        let span = expression.span();
        match &mut expression.expression_type {
            ExpressionType::IntLiteral(_) => PineType::Integer,
            ExpressionType::FloatLiteral(_) => PineType::Float,
            ExpressionType::BoolLiteral(_) => PineType::Bool,
            ExpressionType::StringLiteral(_) => PineType::String,
            ExpressionType::Identifier(identifier) => self.visit_identifier(identifier),
            ExpressionType::Unary(op, expression) => {
                let t = self.visit_expression(expression);

                match op.unary_pine_type(t) {
                    Ok(t) => t,
                    Err(e) => panic!("Type Error at {}: {}", span, e),
                }
            }
            ExpressionType::Binary(lhs, op, rhs) => {
                let lhs_type = self.visit_expression(lhs);
                let rhs_type = self.visit_expression(rhs);

                match op.binary_pine_type(lhs_type, rhs_type) {
                    Ok(t) => t,
                    Err(e) => panic!("Type Error at {}: {}", span, e),
                }
            }
        }
    }

    fn visit_identifier(&self, identifier: &mut Identifier) -> PineType {
        identifier.pine_type()
    }

    fn visit_type_node(&self, type_node: &mut TypeNode) -> PineType {
        type_node.pine_type()
    }
}
