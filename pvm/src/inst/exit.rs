use pvm_proc_macros::Inst;
use crate::env::Environment;
use crate::inst::*;
use crate::cast::*;
use crate::parse::{Line, Literal, Parse, Token};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

/// Exits the process.
#[inst(name = "exit", operands = [OperandFormat::Value])]
pub struct ExitInst {
    pub(crate) src: Operand
}

impl Instruction for ExitInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let val = from_u64!(self.src.value(env)?; i32);
        Err(ExitError::exit(val))
    }
}
