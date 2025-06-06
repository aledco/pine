mod error;
pub(crate) mod scoping;
pub(crate) mod typing;
pub(crate) mod ret;
pub(crate) mod modresv;

pub use error::*;
use crate::Ast;

/// Creates a symbol for the identifier.
fn create_symbol(ident: &crate::ast::Ident, scope: &crate::symbol::ScopeRef) -> SemResult<()> {
    let symbol = crate::symbol::Symbol::new(ident.name.clone(), scope.clone());
    match scope.borrow_mut().add(symbol.clone()) {
        Ok(()) => Ok(()),
        Err(()) => Err(SemError::error(format!("identifier {} has already been defined", ident.name), ident.span())),
    }
}
