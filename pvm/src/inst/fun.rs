use pvm_proc_macros::Inst;
use crate::env::Environment;
use crate::inst::*;
use crate::operand::*;
use crate::parse::{Line, Literal, Parse, Token};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

#[inst(name = "fun", operands = [OperandFormat::Label])]
pub struct FunInst {
    pub(crate) lab: Operand,
}

impl Instruction for FunInst {
    fn execute(&mut self, _env: &mut Environment) -> Result<(), Error> {
        Ok(())
    }

    fn initialize(&self, env: &mut Environment, i: usize) -> Result<(), Error> {
        let label = self.lab.label()?;
        if env.fun_labels.contains_key(&label) {
            return Err(ValidateError::label_already_defined(&label));
        }

        env.fun_labels.insert(label, i+1);
        Ok(())
    }
}

// TODO finish below

// #[inst(name = "push", operands = [OperandFormat::Value])]
// pub struct PushInst {
//     pub(crate) src: Operand,
// }
// 
// #[inst(name = "pop", operands = [OperandFormat::Variable])]
// pub struct PopInst {
//     pub(crate) dest: Operand,
// }
// 
// #[inst(name = "call", operands = [OperandFormat::Variable, OperandFormat::Label])]
// pub struct CallInst {
//     pub(crate) dest: Operand,
//     pub(crate) lab: Operand,
// }
// 
// #[inst(name = "callv", operands = [OperandFormat::Label])]
// pub struct CallvInst {
//     pub(crate) lab: Operand,
// }
// 
// #[inst(name = "ret", operands = [OperandFormat::Value])]
// pub struct RetInst {
//     pub(crate) src: Operand,
// }
// 
// #[inst(name = "retv", operands = [])]
// pub struct RetvInst {}

