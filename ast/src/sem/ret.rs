use crate::ast::*;
use crate::sem::{SemError, SemResult};

/// Checks that all functions with return types return values for every path,
/// and inserts void returns for void functions.
pub(crate) fn check(module: &mut Module) -> SemResult<()> {
    for f in &mut module.funs {
        check_fun(f)?;
    }

    Ok(())
}

fn check_fun(fun: &mut Fun) -> SemResult<()> {
    let ret = match &fun.return_ty {
        Some(ty) => ty.ty.clone(),
        None => PineType::Void
    };

    match ret {
        PineType::Void => {
            // insert return at the end of the block
            match fun.block.stmts.last() {
                Some(Stmt::Return(_)) => Ok(()),
                _ => {
                    let ret = Stmt::Return(ReturnStmt::new(None, fun.span()));
                    fun.block.stmts.push(ret);
                    Ok(())
                }
            }
        }
        _ => {
            // ensure all paths return
            if !fun.all_paths_return() {
                Err(SemError::error("not all paths return", fun.span()))?
            } else {
                Ok(())
            }
        }
    }
}

trait AstAllPathsReturn {
    fn all_paths_return(&self) -> bool;
}

impl AstAllPathsReturn for Fun {
    fn all_paths_return(&self) -> bool {
        self.block.all_paths_return()
    }
}

impl AstAllPathsReturn for IfStmt {
    fn all_paths_return(&self) -> bool {
        match &self.else_block {
            Some(b) => {
                if !b.all_paths_return() {
                    return false;
                }

                self.then_blocks.iter().all(|b| b.all_paths_return())
            },
            None => false
        }
    }
}

impl AstAllPathsReturn for WhileStmt {
    fn all_paths_return(&self) -> bool {
        self.block.all_paths_return()
    }
}

impl AstAllPathsReturn for Block {
    fn all_paths_return(&self) -> bool {
        for s in &self.stmts {
            if s.all_paths_return() {
                return true;
            }
        }

        false
    }
}

impl AstAllPathsReturn for Stmt {
    fn all_paths_return(&self) -> bool {
        match self {
            Stmt::If(s) => s.all_paths_return(),
            Stmt::While(s) => s.all_paths_return(),
            Stmt::Return(_) => true,
            Stmt::Block(b) => b.all_paths_return(),
            _ => false
        }
    }
}
