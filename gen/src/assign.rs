use crate::temp::TempStore;

// TODO need to free temps when possilbe

pub(crate) fn assign(program: &mut ast::Program) {
    let mut temp_store = TempStore::new();
    program.main_module.assign(&mut temp_store)
}

trait AstAssign {
    fn assign(&mut self, temp_store: &mut TempStore);
}

impl AstAssign for ast::Module {
    fn assign(&mut self, temp_store: &mut TempStore) {
        for f in &mut self.funs {
            f.assign(temp_store);
        }
    }
}

impl AstAssign for ast::Fun {
    fn assign(&mut self, temp_store: &mut TempStore) {
        self.block.assign(temp_store);
    }
}

impl AstAssign for ast::LetStmt {
    fn assign(&mut self, temp_store: &mut TempStore) {
        self.expr.assign(temp_store);
    }
}

impl AstAssign for ast::SetStmt {
    fn assign(&mut self, temp_store: &mut TempStore) {
        self.expr.assign(temp_store);
    }
}

impl AstAssign for ast::IfStmt {
    fn assign(&mut self, temp_store: &mut TempStore) {
        for c in &mut self.conds {
            c.assign(temp_store);
        }

        for b in &mut self.then_blocks {
            b.assign(temp_store);
        }

        if let Some(b) = &mut self.else_block {
            b.assign(temp_store);
        }
    }
}

impl AstAssign for ast::WhileStmt {
    fn assign(&mut self, temp_store: &mut TempStore) {
        self.cond.assign(temp_store);
        self.block.assign(temp_store);
    }
}

impl AstAssign for ast::ReturnStmt {
    fn assign(&mut self, temp_store: &mut TempStore) {
        if let Some(e) = &mut self.expr {
            e.assign(temp_store);
        }
    }
}

impl AstAssign for ast::ExprStmt {
    fn assign(&mut self, temp_store: &mut TempStore) {
        self.expr.assign(temp_store);
    }
}

impl AstAssign for ast::Block {
    fn assign(&mut self, temp_store: &mut TempStore) {
        for s in &mut self.stmts {
            s.assign(temp_store);
        }
    }
}

impl AstAssign for ast::Stmt {
    fn assign(&mut self, temp_store: &mut TempStore) {
        match self {
            ast::Stmt::Let(s) => s.assign(temp_store),
            ast::Stmt::Set(s) => s.assign(temp_store),
            ast::Stmt::If(s) => s.assign(temp_store),
            ast::Stmt::While(s) => s.assign(temp_store),
            ast::Stmt::Return(s) => s.assign(temp_store),
            ast::Stmt::Expr(s) => s.assign(temp_store),
            ast::Stmt::Block(s) => s.assign(temp_store),
        }
    }
}

impl AstAssign for ast::IntLitExpr {
    fn assign(&mut self, temp_store: &mut TempStore) {
        let t = temp_store.temp();
        self.dest = pvm::Operand::Variable(t)
    }
}

impl AstAssign for ast::FloatLitExpr {
    fn assign(&mut self, temp_store: &mut TempStore) {
        let t = temp_store.temp();
        self.dest = pvm::Operand::Variable(t)
    }
}

impl AstAssign for ast::BoolLitExpr {
    fn assign(&mut self, temp_store: &mut TempStore) {
        let t = temp_store.temp();
        self.dest = pvm::Operand::Variable(t)
    }
}

impl AstAssign for ast::StringLitExpr {
    fn assign(&mut self, temp_store: &mut TempStore) {
        let t = temp_store.temp();
        self.dest = pvm::Operand::Variable(t)
    }
}

impl AstAssign for ast::IdentExpr {
    fn assign(&mut self, _temp_store: &mut TempStore) {
        self.dest = self.ident.symbol.borrow().dest.clone();
    }
}

impl AstAssign for ast::NewObjectExpr {
    fn assign(&mut self, temp_store: &mut TempStore) {
        for f in &mut self.field_inits {
            f.expr.assign(temp_store);
            let t = temp_store.temp();
            f.dest = pvm::Operand::Variable(t)
        }

        let t = temp_store.temp();
        self.dest = pvm::Operand::Variable(t)
    }
}

impl AstAssign for ast::FieldAccessExpr {
    fn assign(&mut self, temp_store: &mut TempStore) {
        todo!()
    }
}

impl AstAssign for ast::CallExpr {
    fn assign(&mut self, temp_store: &mut TempStore) {
        self.fun.assign(temp_store);
        for a in &mut self.args {
            a.assign(temp_store);
        }

        let t = temp_store.temp();
        self.dest = pvm::Operand::Variable(t)
    }
}

impl AstAssign for ast::UnaryExpr {
    fn assign(&mut self, temp_store: &mut TempStore) {
        self.expr.assign(temp_store);
        let t = temp_store.temp();
        self.dest = pvm::Operand::Variable(t)
    }
}

impl AstAssign for ast::BinaryExpr {
    fn assign(&mut self, temp_store: &mut TempStore) {
        self.left.assign(temp_store);
        self.right.assign(temp_store);
        let t = temp_store.temp();
        self.dest = pvm::Operand::Variable(t)
    }
}

impl AstAssign for ast::Expr {
    fn assign(&mut self, temp_store: &mut TempStore) {
        match self {
            ast::Expr::IntLit(e) => e.assign(temp_store),
            ast::Expr::FloatLit(e) => e.assign(temp_store),
            ast::Expr::BoolLit(e) => e.assign(temp_store),
            ast::Expr::StringLit(e) => e.assign(temp_store),
            ast::Expr::Ident(e) => e.assign(temp_store),
            ast::Expr::NewObject(e) => e.assign(temp_store),
            ast::Expr::FieldAccess(e) => e.assign(temp_store),
            ast::Expr::Call(e) => e.assign(temp_store),
            ast::Expr::Unary(e) => e.assign(temp_store),
            ast::Expr::Binary(e) => e.assign(temp_store),
        }
    }
}
