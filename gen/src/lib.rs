mod codegen;
mod error;
mod temp;
mod assign;

pub fn codegen(program: &mut ast::Program) -> Vec<Box<dyn pvm::Instruction>> {
    assign::assign(program);
    codegen::codegen(program)
}
