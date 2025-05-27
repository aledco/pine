use crate::ast::*;
use crate::sem::SemResult;

/// Annotates global types.
pub(crate) fn global(program: &mut Program) -> SemResult<()> {
    program.main_module.visit()?;
    Ok(())
}

trait AstTyping {
    fn visit(&mut self) -> SemResult<PineType>;
}

impl AstTyping for Module {
    fn visit(&mut self) -> SemResult<PineType> {
        for o in &mut self.objs {
            o.visit()?;
        }
        
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
            let p_type = p.ty.visit()?;
            param_types.push(p_type);
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

impl AstTyping for Object {
    fn visit(&mut self) -> SemResult<PineType> {
        let mut field_types = vec![];
        for f in &mut self.fields {
            let sym = f.ident.symbol.clone();
            let ty = f.ty.visit()?;
            f.ident.symbol.borrow_mut().pine_type = ty.clone();
            field_types.push((sym, ty));
        }

        let obj_type = PineType::Object { fields: field_types };
        self.ident.symbol.borrow_mut().pine_type = obj_type.clone();
        Ok(obj_type)
    }
}

impl AstTyping for Ty {
    fn visit(&mut self) -> SemResult<PineType> {
        Ok(self.ty.clone())
    }
}
