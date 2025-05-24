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

pub use operand::*;
pub use bin_op::*;
pub use un_op::*;
pub use print::*;
pub use read::*;
pub use jump::*;
pub use label::*;
pub use alloc::*;
pub use load::*;
pub use store::*;
pub use fun::*;
pub use exit::*;
pub use error::*;
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
