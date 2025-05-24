mod codegen;
mod append;
mod context;

pub(crate) use codegen::codegen;

type Inst = Box<dyn pvm::Instruction>;
type InstVec = Vec<Inst>;
