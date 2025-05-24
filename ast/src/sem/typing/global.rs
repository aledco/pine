use crate::ast::*;
use crate::sem::{SemError, SemResult};

/// Annotates global types.
pub(crate) fn global(program: &mut Program) -> SemResult<()> {
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

        self.main = match self.scope().borrow().lookup("main") {
            Some(main_symbol) => main_symbol,
            None => return Err(SemError::error("no main function found", self.span())),
        };

        match &self.main.borrow().pine_type {
            PineType::Function { ret, .. } => { // TODO ensure params make sense too
                if **ret != PineType::Void && **ret != PineType::Integer {
                    return Err(SemError::error("main must return void or int", self.span()))
                }
            },
            _ => return Err(SemError::error("main must be a function", self.span()))
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
        Ok(function_type)
    }
}

impl AstTyping for Param {
    fn visit(&mut self) -> SemResult<PineType> {
        let param_type = self.ty.visit()?;
        self.ident.symbol.borrow_mut().pine_type = param_type.clone();
        Ok(param_type)
    }
}

impl AstTyping for Ty {
    fn visit(&mut self) -> SemResult<PineType> {
        Ok(self.ty.clone())
    }
}
