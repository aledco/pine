mod bin_op;
mod print;
mod jump;
mod label;
mod un_op;
mod alloc;
mod load;
mod store;
mod error;
mod fun;
mod operand;
mod exit;
mod read;

pub(crate) use operand::*;
pub(crate) use bin_op::*;
pub(crate) use un_op::*;
pub(crate) use print::*;
pub(crate) use read::*;
pub(crate) use jump::*;
pub(crate) use label::*;
pub(crate) use alloc::*;
pub(crate) use load::*;
pub(crate) use store::*;
pub(crate) use fun::*;
pub(crate) use exit::*;
pub(crate) use error::*;
use crate::env::Environment;
use crate::error::Error;
use std::fmt::{Debug, Display};

/// Defines the traits of an instruction.
pub trait Instruction: Validate + Debug + Display {
    /// Executes the instruction.
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error>;

    /// Increments the instruction pointer.
    fn inc_inst_ptr(&self, env: &mut Environment) -> Result<(), Error> {
        env.inst_ptr += 1;
        Ok(())
    }

    /// Initializes the instruction.
    fn initialize(&self, _env: &mut Environment, _i: usize) -> Result<(), Error> { Ok(()) }
}

/// The validation trait.
pub trait Validate {
    /// Validates the instruction.
    fn validate(&self) -> Result<(), Error>;
}
