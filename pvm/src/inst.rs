use std::fmt::{Display, Formatter};
use crate::env::Environment;
use crate::operand::*;

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

pub trait Instruction {
    fn execute(&mut self, context: &mut Environment);

    fn inc_inst_ptr(&self, context: &mut Environment) {
        context.inst_ptr += 1;
    }
}

// TODO types of instructions: arithmetic, data, branch

#[arithmetic(2)]
pub struct AddInst {}

impl Instruction for AddInst {
    fn execute(&mut self, context: &mut Environment) {
        let val1 = self.src1.value().expect("src1 has no value");
        let val2 = self.src2.value().expect("src2 has no value");
        self.dest.set_value(val1 + val2);
    }
}

impl Display for AddInst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {} + {}", self.dest, self.src1, self.src2)
    }
}

pub struct SubInst {
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

impl Instruction for SubInst {
    fn execute(&mut self, context: &mut Environment) {
        todo!()
    }
}

pub struct MulInst {
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

impl Instruction for MulInst {
    fn execute(&mut self, context: &mut Environment) {
        todo!()
    }
}

pub struct DivInst {
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

impl Instruction for DivInst {
    fn execute(&mut self, context: &mut Environment) {
        todo!()
    }
}

pub struct ModInst {
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

impl Instruction for ModInst {
    fn execute(&mut self, context: &mut Environment) {
        todo!()
    }
}

pub struct PowInst {
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

impl Instruction for PowInst {
    fn execute(&mut self, context: &mut Environment) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[should_panic]
    fn test_add_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let _ = AddInst::new(d, s1, s2);
    }

    #[test]
    fn test_add_inst() {
        let d = Operand::Variable(String::from("x"), None);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let mut inst = AddInst::new(d, s1, s2);
        let mut context = Environment::new(32);
        inst.execute(&mut context);
        inst.inc_inst_ptr(&mut context);
        assert_eq!(inst.dest.value(), Some(5));
        assert_eq!(context.inst_ptr, 1);
    }
}