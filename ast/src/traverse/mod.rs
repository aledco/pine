mod scoping;
mod typing;
 
use crate::ast::*;
use crate::traverse::scoping::AstScopeTraverser;
use crate::traverse::typing::AstTypeTraverser;

pub trait Traverse {
    fn traverse(&mut self, program: &mut Program);
}

pub fn traverse(program: &mut Program) {
    let mut traversals: Vec<Box<dyn Traverse>> = vec![
        AstScopeTraverser::new(),
        AstTypeTraverser::new()
    ];
    
    traversals
        .iter_mut()
        .for_each(|t| t.traverse(program));
}
