use crate::ast::*;
use crate::sem::error::{SemError, SemResult};
use crate::symbol::*;
use crate::sem::create_symbol;

/// Annotates global scopes.
pub(crate) fn global(program: &mut Program) -> SemResult<()> {
    let global_scope = Scope::new_global();
    program.main_module.visit(global_scope)?;
    Ok(())
}

trait AstScoping {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()>;
}

impl AstScoping for Module {
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

impl AstScoping for Object {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.set_scope(scope.clone());

        create_symbol(&self.ident, &scope)?;
        self.ident.visit(scope.clone())?;

        let field_scope = Scope::new_local(scope.clone());
        for f in &mut self.fields {
            f.visit(field_scope.clone())?;
        }

        Ok(())
    }
}

impl AstScoping for Field {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.set_scope(scope.clone());
        create_symbol(&self.ident, &scope)?;
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
