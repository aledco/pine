mod codegen;
mod error;
mod temp;
mod assign;
mod offset;

pub fn codegen(program: &mut ast::Program) -> Vec<Box<dyn pvm::Instruction>> {
    assign::assign(program);
    offset::offset(program);
    codegen::codegen(program)
}
