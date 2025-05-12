mod bin_op;
mod print;
mod jump;
mod label;
mod un_op;

pub(crate) use bin_op::*;
pub(crate) use un_op::*;
pub(crate) use print::*;
pub(crate) use jump::*;
pub(crate) use label::*;
use crate::env::Environment;
use crate::operand::*;
use std::fmt::{Debug, Display};

pub trait Instruction: Debug + Display {
    fn execute(&mut self, env: &mut Environment) -> Result<(), String>;

    fn inc_inst_ptr(&self, env: &mut Environment) -> Result<(), String> {
        env.inst_ptr += 1;
        Ok(())
    }

    fn defined_label(&self) -> Option<String> {
        None
    }
    
    fn defined_var(&self) -> Option<Operand> {
        None
    }

    fn used_vars(&self) -> Vec<Operand> {
        Vec::new()
    }
    
    fn validate(&self) -> Result<(), String> { Ok(()) }
}
