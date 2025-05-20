use crate::Traverse;
use ast::Ast;

pub struct AstScopeTraverser;

impl Traverse for AstScopeTraverser {
    fn traverse(&mut self, program: &mut ast::Program) {
        let global_scope = ast::Scope::new_global();
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

    fn visit_function(&self, function: &mut ast::Function, scope: ast::ScopeRef) {
        function.set_scope(scope.clone());

        create_symbol(function.identifier.as_mut(), &scope);
        self.visit_identifier(function.identifier.as_mut(), scope.clone());

        let param_scope = ast::Scope::new_local(scope.clone());
        function
            .params
            .iter_mut()
            .for_each(|p| self.visit_param(p, param_scope.clone()));

        if let Some(return_type_node) = &mut function.return_type_node {
            self.visit_type_node(return_type_node.as_mut(), scope.clone());
        }

        let body_scope = ast::Scope::new_local(scope.clone());
        self.visit_statement(function.body.as_mut(), body_scope);
    }

    fn visit_param(&self, param: &mut ast::Param, scope: ast::ScopeRef) {
        param.set_scope(scope.clone());

        create_symbol(param.identifier.as_mut(), &scope);
        self.visit_identifier(param.identifier.as_mut(), scope.clone());

        self.visit_type_node(param.type_node.as_mut(), scope.clone());
    }

    fn visit_statement(&self, statement: &mut ast::Statement, scope: ast::ScopeRef) {
        statement.set_scope(scope.clone());

        match &mut statement.statement_type {
            ast::StatementType::Let(identifier, type_node, expression) => {
                create_symbol(identifier.as_mut(), &scope);
                self.visit_identifier(identifier.as_mut(), scope.clone());

                if let Some(type_node) = type_node {
                    self.visit_type_node(type_node, scope.clone());
                }

                self.visit_expression(expression.as_mut(), scope.clone());
            }
            ast::StatementType::Set(identifier, expression) => {
                self.visit_identifier(identifier.as_mut(), scope.clone());
                self.visit_expression(expression.as_mut(), scope.clone());
            }
            ast::StatementType::If(condition, if_body, else_body) => {
                self.visit_expression(condition.as_mut(), scope.clone());
                self.visit_statement(if_body.as_mut(), ast::Scope::new_local(scope.clone()));
                if let Some(else_body) = else_body {
                    self.visit_statement(else_body.as_mut(), ast::Scope::new_local(scope.clone()));
                }
            }
            ast::StatementType::While(condition, body) => {
                self.visit_expression(condition.as_mut(), scope.clone());
                self.visit_statement(body.as_mut(), ast::Scope::new_local(scope.clone()));
            }
            ast::StatementType::Return(expression) => {
                if let Some(expression) = expression {
                    self.visit_expression(expression.as_mut(), scope.clone());
                }
            }
            ast::StatementType::Expression(expression) => {
                self.visit_expression(expression.as_mut(), scope.clone());
            }
            ast::StatementType::Block(statements) => {
                statements
                    .iter_mut()
                    .for_each(|s| self.visit_statement(s, scope.clone()));
            }
        }
    }

    fn visit_expression(&self, expression: &mut ast::Expression, scope: ast::ScopeRef) {
        expression.set_scope(scope.clone());

        match &mut expression.expression_type {
            ast::ExpressionType::IntLiteral(_) => {}
            ast::ExpressionType::FloatLiteral(_) => {}
            ast::ExpressionType::BoolLiteral(_) => {}
            ast::ExpressionType::StringLiteral(_) => {}
            ast::ExpressionType::Identifier(identifier) => {
                self.visit_identifier(identifier.as_mut(), scope.clone());
            }
            ast::ExpressionType::Unary(_, expression) => {
                self.visit_expression(expression.as_mut(), scope.clone());
            }
            ast::ExpressionType::Binary(lhs, _, rhs) => {
                self.visit_expression(lhs, scope.clone());
                self.visit_expression(rhs, scope.clone());
            }
        }
    }

    fn visit_identifier(&self, identifier: &mut ast::Identifier, scope: ast::ScopeRef) {
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

    fn visit_type_node(&self, type_node: &mut ast::TypeNode, scope: ast::ScopeRef) {
        type_node.set_scope(scope.clone());
    }
}

fn create_symbol(identifier: &mut ast::Identifier, scope: &ast::ScopeRef) {
    let symbol = ast::Symbol::new(identifier.name.clone());
    let result = scope.borrow_mut().add(symbol);
    if result.is_err() {
        panic!(
            "Name Error at {}: identifier {} already exists",
            identifier.span(),
            identifier.name
        )
    }
}
