use crate::env::Environment;
use crate::operand::*;

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

pub trait Instruction {
    fn execute(&self, context: &mut Environment);

    fn inc_inst_ptr(&self, context: &mut Environment) {
        context.inst_ptr += 1;
    }

    fn to_string(&self);

    fn to_verbose_string(&self);
}

// TODO types of instructions: arithmetic, data, branch

#[arithmetic(2)]
pub struct AddInst {}

impl Instruction for AddInst {
    fn execute(&self, context: &mut Environment) {
        todo!()
    }

    fn to_string(&self) {
        todo!()
    }

    fn to_verbose_string(&self) {
        todo!()
    }
}

pub struct SubInst {
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

impl Instruction for SubInst {
    fn execute(&self, context: &mut Environment) {
        todo!()
    }

    fn to_string(&self) {
        todo!()
    }

    fn to_verbose_string(&self) {
        todo!()
    }
}

pub struct MulInst {
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

impl Instruction for MulInst {
    fn execute(&self, context: &mut Environment) {
        todo!()
    }

    fn to_string(&self) {
        todo!()
    }

    fn to_verbose_string(&self) {
        todo!()
    }
}

pub struct DivInst {
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

impl Instruction for DivInst {
    fn execute(&self, context: &mut Environment) {
        todo!()
    }

    fn to_string(&self) {
        todo!()
    }

    fn to_verbose_string(&self) {
        todo!()
    }
}

pub struct ModInst {
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

impl Instruction for ModInst {
    fn execute(&self, context: &mut Environment) {
        todo!()
    }

    fn to_string(&self) {
        todo!()
    }

    fn to_verbose_string(&self) {
        todo!()
    }
}

pub struct PowInst {
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

impl Instruction for PowInst {
    fn execute(&self, context: &mut Environment) {
        todo!()
    }

    fn to_string(&self) {
        todo!()
    }

    fn to_verbose_string(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[should_panic]
    fn test_add_inst_validation() {
        let d = Operand::Constant(Value::Integer(0));
        let s1 = Operand::Constant(Value::Integer(2));
        let s2 = Operand::Constant(Value::Integer(3));
        let _ = AddInst::new(d, s1, s2);
    }

    #[test]
    fn test_add_inst() {
        let d = Operand::Variable(String::from("test"), Value::Integer(1));
        let s1 = Operand::Constant(Value::Integer(2));
        let s2 = Operand::Constant(Value::Integer(3));
        let _ = AddInst::new(d, s1, s2);
        // TODO test execute
    }
}