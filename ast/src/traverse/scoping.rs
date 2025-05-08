use crate::ast::*;
use crate::symbol::*;
use crate::traverse::Traverse;

pub struct AstScopeTraverser;

impl Traverse for AstScopeTraverser {
    fn traverse(&mut self, program: &mut Program) {
        let global_scope = Scope::new_global();
        program
            .functions
            .iter_mut()
            .for_each(|f| self.visit_function(f, global_scope.clone()));
    }
}

impl AstScopeTraverser {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }

    fn visit_function(&self, function: &mut Function, scope: ScopeRef) {
        function.set_scope(scope.clone());

        create_symbol(function.identifier.as_mut(), &scope);
        self.visit_identifier(function.identifier.as_mut(), scope.clone());

        let param_scope = Scope::new_local(scope.clone());
        function
            .params
            .iter_mut()
            .for_each(|p| self.visit_param(p, param_scope.clone()));

        if let Some(return_type_node) = &mut function.return_type_node {
            self.visit_type_node(return_type_node.as_mut(), scope.clone());
        }

        let body_scope = Scope::new_local(scope.clone());
        self.visit_statement(function.body.as_mut(), body_scope);
    }

    fn visit_param(&self, param: &mut Param, scope: ScopeRef) {
        param.set_scope(scope.clone());

        create_symbol(param.identifier.as_mut(), &scope);
        self.visit_identifier(param.identifier.as_mut(), scope.clone());

        self.visit_type_node(param.type_node.as_mut(), scope.clone());
    }

    fn visit_statement(&self, statement: &mut Statement, scope: ScopeRef) {
        statement.set_scope(scope.clone());

        match &mut statement.statement_type {
            StatementType::Let(identifier, type_node, expression) => {
                create_symbol(identifier.as_mut(), &scope);
                self.visit_identifier(identifier.as_mut(), scope.clone());

                if let Some(type_node) = type_node {
                    self.visit_type_node(type_node, scope.clone());
                }

                self.visit_expression(expression.as_mut(), scope.clone());
            }
            StatementType::Set(identifier, expression) => {
                self.visit_identifier(identifier.as_mut(), scope.clone());
                self.visit_expression(expression.as_mut(), scope.clone());
            }
            StatementType::If(condition, if_body, else_body) => {
                self.visit_expression(condition.as_mut(), scope.clone());
                self.visit_statement(if_body.as_mut(), Scope::new_local(scope.clone()));
                if let Some(else_body) = else_body {
                    self.visit_statement(else_body.as_mut(), Scope::new_local(scope.clone()));
                }
            }
            StatementType::While(condition, body) => {
                self.visit_expression(condition.as_mut(), scope.clone());
                self.visit_statement(body.as_mut(), Scope::new_local(scope.clone()));
            }
            StatementType::Return(expression) => {
                if let Some(expression) = expression {
                    self.visit_expression(expression.as_mut(), scope.clone());
                }
            }
            StatementType::Expression(expression) => {
                self.visit_expression(expression.as_mut(), scope.clone());
            }
            StatementType::Block(statements) => {
                statements
                    .iter_mut()
                    .for_each(|s| self.visit_statement(s, scope.clone()));
            }
        }
    }

    fn visit_expression(&self, expression: &mut Expression, scope: ScopeRef) {
        expression.set_scope(scope.clone());

        match &mut expression.expression_type {
            ExpressionType::IntLiteral(_) => {}
            ExpressionType::FloatLiteral(_) => {}
            ExpressionType::BoolLiteral(_) => {}
            ExpressionType::StringLiteral(_) => {}
            ExpressionType::Identifier(identifier) => {
                self.visit_identifier(identifier.as_mut(), scope.clone());
            }
            ExpressionType::Unary(_, expression) => {
                self.visit_expression(expression.as_mut(), scope.clone());
            }
            ExpressionType::Binary(lhs, _, rhs) => {
                self.visit_expression(lhs, scope.clone());
                self.visit_expression(rhs, scope.clone());
            }
        }
    }

    fn visit_identifier(&self, identifier: &mut Identifier, scope: ScopeRef) {
        identifier.set_scope(scope.clone());
        identifier.symbol = match scope.borrow().lookup(&identifier.name) {
            Some(s) => s,
            None => panic!(
                "Name Error at {}: couldn't find {} in scope",
                identifier.span(),
                identifier.name
            ),
        }
    }

    fn visit_type_node(&self, type_node: &mut TypeNode, scope: ScopeRef) {
        type_node.set_scope(scope.clone());
    }
}

fn create_symbol(identifier: &mut Identifier, scope: &ScopeRef) {
    let symbol = Symbol::new(identifier.name.clone());
    let result = scope.borrow_mut().add(symbol);
    if result.is_err() {
        panic!(
            "Name Error at {}: identifier {} already exists",
            identifier.span(),
            identifier.name
        )
    }
}
