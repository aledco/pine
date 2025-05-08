use crate::env::Environment;

pub trait Instruction {
    fn execute(&self, context: &mut Environment);

    fn inc_inst_ptr(&self, context: &mut Environment) {
        context.inst_ptr += 1;
    }

    fn to_string(&self);

    fn to_verbose_string(&self);
}

pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Address(usize)
}

pub enum Operand {
    Constant(Value),
    Variable(String, Value),
    Label(String),
}

pub struct AddInst { // TODO use macros for three operand insts
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

impl AddInst { // TODO use macro to do this?
    pub fn new(dest: Operand, src1: Operand, src2: Operand) -> Self {
        if !matches!(dest, Operand::Variable(_, _)) {
            panic!("destination must be a variable");
        }

        if !matches!(src1, Operand::Constant(_)) || !matches!(src1, Operand::Variable(_, _)) {
            panic!("src1 must be a variable or constant");
        }

        if !matches!(src2, Operand::Constant(_)) || !matches!(src2, Operand::Variable(_, _)) {
            panic!("src2 must be a variable or constant");
        }

        Self {
            dest,
            src1,
            src2
        }
    }
}

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
