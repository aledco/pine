mod bin_op;
mod print;
mod jump;
mod label;
mod un_op;
mod alloc;
mod load;
mod store;

pub(crate) use bin_op::*;
pub(crate) use un_op::*;
pub(crate) use print::*;
pub(crate) use jump::*;
pub(crate) use label::*;
pub(crate) use alloc::*;
pub(crate) use load::*;
pub(crate) use store::*;
use crate::env::Environment;
use std::fmt::{Debug, Display};

pub trait Instruction: Validate + Debug + Display {
    fn execute(&mut self, env: &mut Environment) -> Result<(), String>;

    fn inc_inst_ptr(&self, env: &mut Environment) -> Result<(), String> {
        env.inst_ptr += 1;
        Ok(())
    }

    fn defined_label(&self) -> Option<String> {
        None
    }
}

pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}
