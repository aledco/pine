use crate::ast::*;
use crate::sem::{SemError, SemResult};
use crate::symbol::*;

// Simple Alg: https://pfudke.wordpress.com/2014/11/20/hindley-milner-type-inference-a-practical-example-2/
// Efficient Alg: https://okmij.org/ftp/ML/generalization.html

pub(crate) fn typing(program: &mut Program) -> SemResult<()> {
    program.visit()?;
    Ok(())
}

trait AstTyping {
    fn visit(&mut self) -> SemResult<PineType>;
}

impl AstTyping for Program {
    fn visit(&mut self) -> SemResult<PineType> {
        for f in &mut self.funs {
            f.visit()?;
        }

        Ok(PineType::Void)
    }
}

impl AstTyping for Fun {
    fn visit(&mut self) -> SemResult<PineType> {
        let mut param_types: Vec<PineType> = vec![];
        for p in &mut self.params {
            param_types.push(p.visit()?);
        }

        let return_type = match &mut self.return_ty {
            Some(t) => t.visit()?,
            None => PineType::Void,
        };

        let function_type = PineType::Function {
            params: param_types,
            ret: Box::new(return_type),
        };

        self.ident.symbol.borrow_mut().pine_type = function_type.clone();

        self.ident.visit()?;
        self.block.visit()?;

        // TODO ensure return type is valid
        // TODO ensure all paths return if return type is not void

        Ok(function_type)
    }
}

impl AstTyping for Param {
    fn visit(&mut self) -> SemResult<PineType> {
        let param_type = self.ty.visit()?;
        self.ident.symbol.borrow_mut().pine_type = param_type.clone();
        self.ident.visit()?;
        Ok(param_type)
    }
}

impl AstTyping for LetStmt {
    fn visit(&mut self) -> SemResult<PineType> {
        let e_type = self.expr.visit()?;
        self.ident.symbol.borrow_mut().pine_type = if let Some(ty) = &mut self.ty {
            let n_type = ty.visit()?;
            if n_type != e_type {
                return Err(SemError::error("types do not match", self.span()));
            }

            n_type
        } else {
            e_type
        };

        self.ident.visit()?;
        Ok(PineType::Void)
    }
}

impl AstTyping for SetStmt {
    fn visit(&mut self) -> SemResult<PineType> {
        let e_type = self.expr.visit()?;
        let i_type = self.ident.visit()?;
        if e_type != i_type {
           Err(SemError::error("types do not match", self.span()))
        } else {
            Ok(PineType::Void)
        }
    }
}

impl AstTyping for IfStmt {
    fn visit(&mut self) -> SemResult<PineType> {
        let c_type = self.cond.visit()?;
        if c_type != PineType::Bool {
            return Err(SemError::error("condition must have type bool", self.span()))
        }

        self.then_block.visit()?;
        if let Some(else_block) = &mut self.else_block {
            else_block.visit()?;
        }

        Ok(PineType::Void)
    }
}

impl AstTyping for WhileStmt {
    fn visit(&mut self) -> SemResult<PineType> {
        let c_type = self.cond.visit()?;
        if c_type != PineType::Bool {
            return Err(SemError::error("condition must have type bool", self.span()))
        }

        self.block.visit()?;
        Ok(PineType::Void)
    }
}

impl AstTyping for ReturnStmt {
    fn visit(&mut self) -> SemResult<PineType> {
        let fun_sym = match self.scope().borrow().owning_fun() {
            Some(f) => f,
            None => return Err(SemError::error("could not find enclosing function", self.span()))
        };
        let fun_ret_ty = match &fun_sym.borrow().pine_type {
            PineType::Function { ret, .. } => ret.as_ref().clone(),
            _ => return Err(SemError::error("could not find enclosing function", self.span()))
        };
        
        println!("ret_ty = {:?}", fun_ret_ty);
        
        if let Some(expr) = &mut self.expr {
            let e_type = expr.visit()?;
            println!("e_ty = {:?}", e_type);
            if e_type != fun_ret_ty {
                return Err(SemError::error("types do not match", self.span()))
            }
        } else {
            if fun_ret_ty != PineType::Void {
                return Err(SemError::error("types do not match", self.span()))
            }
        }

        Ok(PineType::Void)
    }
}

impl AstTyping for ExprStmt {
    fn visit(&mut self) -> SemResult<PineType> {
        self.expr.visit()?;
        Ok(PineType::Void)
    }
}

impl AstTyping for Block {
    fn visit(&mut self) -> SemResult<PineType> {
        for s in &mut self.stmts {
            s.visit()?;
        }

        Ok(PineType::Void)
    }
}

impl AstTyping for Stmt {
    fn visit(&mut self) -> SemResult<PineType> {
        match self {
            Stmt::Let(s) => s.visit(),
            Stmt::Set(s) => s.visit(),
            Stmt::If(s) => s.visit(),
            Stmt::While(s) => s.visit(),
            Stmt::Return(s) => s.visit(),
            Stmt::Expr(s) => s.visit(),
            Stmt::Block(s) => s.visit()
        }
    }
}

impl AstTyping for IntLitExpr {
    fn visit(&mut self) -> SemResult<PineType> {
        Ok(PineType::Integer)
    }
}

impl AstTyping for FloatLitExpr {
    fn visit(&mut self) -> SemResult<PineType> {
        Ok(PineType::Float)
    }
}

impl AstTyping for BoolLitExpr {
    fn visit(&mut self) -> SemResult<PineType> {
        Ok(PineType::Bool)
    }
}

impl AstTyping for StringLitExpr {
    fn visit(&mut self) -> SemResult<PineType> {
        Ok(PineType::String)
    }
}

impl AstTyping for IdentExpr {
    fn visit(&mut self) -> SemResult<PineType> {
        self.ident.visit()
    }
}

impl AstTyping for UnaryExpr {
    fn visit(&mut self) -> SemResult<PineType> {
        let t = self.expr.visit()?;
        match self.op.unary_pine_type(t) {
            Ok(t) => Ok(t),
            Err(e) => Err(SemError::error(e, self.span()))
        } // TODO use type env?
    }
}

impl AstTyping for BinaryExpr {
    fn visit(&mut self) -> SemResult<PineType> {
        let l = self.left.visit()?;
        let r = self.right.visit()?;
        match self.op.binary_pine_type(l, r) {
            Ok(t) => Ok(t),
            Err(e) => Err(SemError::error(e, self.span()))
        } // TODO use type env?
    }
}

impl AstTyping for Expr {
    fn visit(&mut self) -> SemResult<PineType> {
        match self {
            Expr::IntLit(e) => e.visit(),
            Expr::FloatLit(e) => e.visit(),
            Expr::BoolLit(e) => e.visit(),
            Expr::StringLit(e) => e.visit(),
            Expr::Ident(e) => e.visit(),
            Expr::Unary(e) => e.visit(),
            Expr::Binary(e) => e.visit(),
        }
    }
}

impl AstTyping for Ident {
    fn visit(&mut self) -> SemResult<PineType> {
        Ok(self.symbol.borrow().pine_type.clone())
    }
}

impl AstTyping for Ty {
    fn visit(&mut self) -> SemResult<PineType> {
        Ok(self.ty())
    }
}