use crate::ast::*;
use crate::sem::error::{SemError, SemResult};
use crate::symbol::*;

pub(crate) fn scoping(program: &mut Program) -> SemResult<()> {
    let global_scope = Scope::new_global();
    program.visit(global_scope)?;
    Ok(())
}

trait AstScoping {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()>;
}

impl AstScoping for Program {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
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

        // crate the param scope and visit the params
        let param_scope = Scope::new_fun(scope.clone(), self.ident.symbol.clone());
        for p in &mut self.params {
            p.visit(param_scope.clone())?;
        }

        // visit the return type
        if let Some(ret_ty) = &mut self.return_ty {
            ret_ty.visit(param_scope.clone())?;
        }

        // create the block scope and visit the block
        let block_scope = Scope::new_fun(scope.clone(), self.ident.symbol.clone());
        self.block.visit(block_scope.clone())?;
        Ok(())
    }
}

impl AstScoping for Param {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.set_scope(scope.clone());
        create_symbol(&self.ident, &scope)?;
        self.ident.visit(scope.clone())?;
        self.ty.visit(scope.clone())?;
        Ok(())
    }
}

impl AstScoping for LetStmt {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        create_symbol(&self.ident, &scope)?;
        self.ident.visit(scope.clone())?;
        if let Some(ty) = &mut self.ty {
            ty.visit(scope.clone())?;
        }

        self.expr.visit(scope.clone())?;
        Ok(())
    }
}

impl AstScoping for SetStmt {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.ident.visit(scope.clone())?;
        self.expr.visit(scope.clone())?;
        Ok(())
    }
}

impl AstScoping for IfStmt {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.cond.visit(scope.clone())?;
        let then_scope = Scope::new_local(scope.clone());
        self.then_block.visit(then_scope)?;
        if let Some(else_block) = &mut self.else_block {
            let else_scope = Scope::new_local(scope.clone());
            else_block.visit(else_scope)?;
        }

        Ok(())
    }
}

impl AstScoping for WhileStmt {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.cond.visit(scope.clone())?;
        let block_scope = Scope::new_local(scope.clone());
        self.block.visit(block_scope)?;
        Ok(())
    }
}

impl AstScoping for ReturnStmt {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        if let Some(expr) = &mut self.expr {
            expr.visit(scope.clone())?;
        }

        Ok(())
    }
}

impl AstScoping for ExprStmt {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.expr.visit(scope.clone())?;
        Ok(())
    }
}

impl AstScoping for Block {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        for s in &mut self.stmts {
            s.visit(scope.clone())?;
        }

        Ok(())
    }
}

impl AstScoping for Stmt {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.set_scope(scope.clone());
        match self {
            Stmt::Let(s) => s.visit(scope),
            Stmt::Set(s) => s.visit(scope),
            Stmt::If(s) => s.visit(scope),
            Stmt::While(s) => s.visit(scope),
            Stmt::Return(s) => s.visit(scope),
            Stmt::Expr(s) => s.visit(scope),
            Stmt::Block(s) => s.visit(scope)
        }
    }
}

impl AstScoping for IntLitExpr {
    fn visit(&mut self, _scope: ScopeRef) -> SemResult<()> {
        Ok(())
    }
}

impl AstScoping for FloatLitExpr {
    fn visit(&mut self, _scope: ScopeRef) -> SemResult<()> {
        Ok(())
    }
}

impl AstScoping for BoolLitExpr {
    fn visit(&mut self, _scope: ScopeRef) -> SemResult<()> {
        Ok(())
    }
}

impl AstScoping for StringLitExpr {
    fn visit(&mut self, _scope: ScopeRef) -> SemResult<()> {
        Ok(())
    }
}

impl AstScoping for IdentExpr {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.ident.visit(scope)?;
        Ok(())
    }
}

impl AstScoping for UnaryExpr {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.expr.visit(scope)?;
        Ok(())
    }
}

impl AstScoping for BinaryExpr {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.left.visit(scope.clone())?;
        self.right.visit(scope.clone())?;
        Ok(())
    }
}

impl AstScoping for Expr {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.set_scope(scope.clone());
        match self {
            Expr::IntLit(e) => e.visit(scope),
            Expr::FloatLit(e) => e.visit(scope),
            Expr::BoolLit(e) => e.visit(scope),
            Expr::StringLit(e) => e.visit(scope),
            Expr::Ident(e) => e.visit(scope),
            Expr::Unary(e) => e.visit(scope),
            Expr::Binary(e) => e.visit(scope),
        }
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

impl AstScoping for Ty {
    fn visit(&mut self, scope: ScopeRef) -> SemResult<()> {
        self.set_scope(scope);
        Ok(())
    }
}

fn create_symbol(ident: &Ident, scope: &ScopeRef) -> SemResult<()> {
    let symbol = Symbol::new(ident.name.clone());
    match scope.borrow_mut().add(symbol.clone()) {
        Ok(()) => Ok(()),
        Err(()) => Err(SemError::error(format!("identifier {} has already been defined", ident.name), ident.span())),
    }
}
