use pvm_proc_macros::Inst;
use crate::env::Environment;
use crate::inst::*;
use crate::operand::*;
use crate::cast::*;
use crate::parse::{Line, Literal, Parse, Token};
use std::fmt::{Debug, Display, Formatter};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

// TODO implement dealloc, add tests, implement load and store

#[inst(name = "alloc", operands = [OperandFormat::Variable, OperandFormat::Value])]
pub struct AllocInst {
    pub dest: Operand,
    pub src: Operand,
}

impl Instruction for AllocInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), String> {
        let size = from_u64!(self.src.value(env)?; usize);
        let addr = to_u64!(env.memory.allocate(size)?);
        self.dest.set_value(addr, env);
        Ok(())
    }
}

impl Display for AllocInst { // TODO can auto derive this in inst too
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", Self::NAME, self.src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_alloc_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let inst = AllocInst::new(d, s1);
        inst.validate().unwrap();
    }
}
