use crate::ast::*;
use crate::symbol::*;
use crate::traverse::AstTraverse;

pub struct AstScopeTraverser;

impl AstTraverse for AstScopeTraverser {
    fn traverse(&self, program: &mut Program) {
        let global_scope = Scope::new_global();
        program
            .functions
            .iter_mut()
            .for_each(|f| self.process(f, global_scope.clone()));
    }
}

impl AstScopeTraverser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process(&self, node: &mut AstNode, scope: ScopeRef) {
        node.scope = scope.clone();
        match &mut node.ast_type {
            AstType::Function {
                identifier,
                params,
                return_type_node,
                body,
            } => {
                create_symbol(identifier, &scope);
                self.process(identifier.as_mut(), scope.clone());
                params
                    .iter_mut()
                    .for_each(|p| self.process(p, scope.clone()));
                if let Some(return_type_node) = return_type_node {
                    self.process(return_type_node.as_mut(), scope.clone());
                }
                self.process(body.as_mut(), scope.clone());
            }
            AstType::Param {
                identifier,
                type_node,
            } => {
                create_symbol(identifier, &scope);
                self.process(identifier.as_mut(), scope.clone());
                self.process(type_node.as_mut(), scope.clone());
            }
            AstType::Block(statements) => {
                statements
                    .iter_mut()
                    .for_each(|s| self.process(s, scope.clone()));
            }
            AstType::IfStatement {
                condition,
                then_body,
                else_body,
            } => {
                self.process(condition.as_mut(), scope.clone());
                self.process(then_body.as_mut(), scope.clone());
                if let Some(else_body) = else_body {
                    self.process(else_body.as_mut(), scope.clone());
                }
            }
            AstType::WhileStatement { condition, body } => {
                self.process(condition.as_mut(), scope.clone());
                self.process(body.as_mut(), scope.clone());
            }
            AstType::ReturnStatement(expression) => {
                if let Some(expression) = expression {
                    self.process(expression, scope.clone());
                }
            }
            AstType::LetStatement {
                identifier,
                type_node,
                expression,
            } => {
                create_symbol(identifier, &scope);
                
                self.process(identifier.as_mut(), scope.clone());
                if let Some(type_node) = type_node {
                    self.process(type_node.as_mut(), scope.clone());
                }
                self.process(expression, scope.clone());
            }
            AstType::SetStatement {
                identifier,
                expression,
            } => {
                self.process(identifier.as_mut(), scope.clone());
                self.process(expression, scope.clone());
            }
            AstType::BinaryExpression { lhs, rhs, .. } => {
                self.process(lhs.as_mut(), scope.clone());
                self.process(rhs, scope.clone());
            }
            AstType::UnaryExpression { expr, ..} => {
                self.process(expr, scope.clone());
            }
            AstType::IdentifierExpression(identifier) => {
                self.process(identifier, scope.clone());
            }
            AstType::Identifier { name, symbol } => {
                match scope.borrow().lookup(name) {
                    Some(s) => *symbol = s,
                    None => panic!("Name Error at {}: couldn't find {} in scope", node.span, name)
                }
            }
            AstType::IntegerExpression(_) => {}
            AstType::FloatExpression(_) => {}
            AstType::StringExpression(_) => {}
            AstType::TypeNode(_) => {}
            AstType::Dummy => {}
        }
    }
}

fn create_symbol(identifier: &mut Box<AstNode>, scope: &ScopeRef) {
    match &identifier.ast_type {
        AstType::Identifier { name, .. } => {
            let symbol = Symbol::new(name.clone());
            scope.borrow_mut().add(symbol);
        }
        _ => panic!()
    }
}
