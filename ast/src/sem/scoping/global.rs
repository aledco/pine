use crate::ast::*;
use crate::sem::error::{SemError, SemResult};
use crate::symbol::*;
use crate::sem::create_symbol;

/// Annotates global scopes.
pub(crate) fn global(program: &mut Program) -> SemResult<()> {
    let global_scope = Scope::new_global();
    program.visit(global_scope)?;
    Ok(())
}

trait AstScoping {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()>;
}

impl AstScoping for Program {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.set_scope(scope.clone());
        for f in &mut self.funs {
            f.visit(scope.clone())?;
        }
        Ok(())
    }
}

impl AstScoping for Fun {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.set_scope(scope.clone());

        // create the symbol and visit the identifier
        create_symbol(&self.ident, &scope)?;
        self.ident.visit(scope.clone())?;
        
        Ok(())
    }
}

impl AstScoping for Ident {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.set_scope(scope.clone());
        self.symbol = match scope.borrow().lookup(&self.name) {
            Some(s) => Ok(s),
            None => Err(SemError::error(format!("identifier {} does not exist in scope", self.name), self.span()))
        }?;
        Ok(())
    }
}
