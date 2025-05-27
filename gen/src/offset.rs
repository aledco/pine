use crate::temp::TempStore;

pub(crate) fn offset(program: &mut ast::Program) {
    program.main_module.offset()
}

trait AstOffset {
    fn offset(&mut self);
}

impl AstOffset for ast::Module {
    fn offset(&mut self) {
        for o in &mut self.objs {
            o.offset();
        }
    }
}

impl AstOffset for ast::Object {
    fn offset(&mut self) {
        let mut curr: usize = 0;
        for f in &self.fields {
            f.ident.symbol.borrow_mut().offset = curr;
            curr += f.ident.symbol.borrow().pine_type.sizeof();
        }
    }
}