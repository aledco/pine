use pvm_proc_macros::Inst;
use crate::env::Environment;
use crate::inst::Instruction;
use crate::operand::*;
use crate::parse::{Line, Literal, Parse, Token};
use std::fmt::{Debug, Display, Formatter};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

#[inst(name = "jump", operands = [OperandFormat::Value])]
pub struct JumpInst {
    pub src: Operand,
}

impl Instruction for JumpInst {
    fn execute(&mut self, _env: &mut Environment) -> Result<(), String> {
        Ok(())
    }

    fn inc_inst_ptr(&self, env: &mut Environment) -> Result<(), String> {
        let label = self.src.label()?;
        let addr = env.labels.get(&label);
        match addr {
            Some(addr) => {
                env.inst_ptr = *addr as usize;
                Ok(())
            },
            None => Err(format!("Label {} does not exist", label))?,
        }
    }

    fn used_vars(&self) -> Vec<Operand> {
        if let Operand::Variable(_) = self.src {
            return vec![self.src.clone()];
        }

        vec![]
    }

    fn validate(&self) -> Result<(), String> {
        if !matches!(self.src, Operand::Label(_)) {
            Err("src must be a label".to_string())
        } else {
            Ok(())
        }
    }
}

impl Display for JumpInst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", Self::NAME, self.src)
    }
}
