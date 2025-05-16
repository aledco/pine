mod bin_op;
mod print;
mod jump;
mod label;
mod un_op;
mod alloc;
mod load;
mod store;
mod error;

pub(crate) use bin_op::*;
pub(crate) use un_op::*;
pub(crate) use print::*;
pub(crate) use jump::*;
pub(crate) use label::*;
pub(crate) use alloc::*;
pub(crate) use load::*;
pub(crate) use store::*;
pub(crate) use error::*;
use crate::env::Environment;
use crate::error::Error;
use std::fmt::{Debug, Display};

pub trait Instruction: Validate + Debug + Display {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error>;

    fn inc_inst_ptr(&self, env: &mut Environment) -> Result<(), Error> {
        env.inst_ptr += 1;
        Ok(())
    }
    
    fn initialize(&self, _env: &mut Environment, _i: usize) -> Result<(), Error> { Ok(()) }
}

pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}
