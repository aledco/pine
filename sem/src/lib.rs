mod scoping;
mod typing;

use crate::scoping::AstScopeTraverser;
use crate::typing::AstTypeTraverser;

trait Traverse {
    fn traverse(&mut self, program: &mut ast::Program);
}

/// Performs semantic analysis traversals on the given Pine program.
/// Currently, only typing and scoping are done.
pub fn traverse(program: &mut ast::Program) {
    let mut traversals: Vec<Box<dyn Traverse>> =
        vec![AstScopeTraverser::new(), AstTypeTraverser::new()];

    traversals.iter_mut().for_each(|t| t.traverse(program));
}
