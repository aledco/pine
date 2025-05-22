use crate::env::MemoryError;
use crate::inst::{ExecuteError, ExitError, ValidateError};
use crate::parse::ParseError;
use std::fmt::{Debug, Display};

/// The PVM error.
#[derive(Clone)]
pub enum Error {
    Parse(ParseError),
    Validate(ValidateError),
    Execute(ExecuteError),
    Exit(ExitError),
    Memory(MemoryError),
    Wrapped(Box<Error>, usize),
}

impl Error {
    pub(crate) fn wrap(&self, i: usize) -> Error {
        match self {
            Error::Validate(_) | Error::Execute(_) | Error::Memory(_) => {
                Error::Wrapped(Box::new(self.clone()), i)
            }
            e => e.clone(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(e) => Display::fmt(&e, f),
            Error::Validate(e) => Display::fmt(&e, f),
            Error::Execute(e) => Display::fmt(&e, f),
            Error::Exit(e) => Display::fmt(&e, f),
            Error::Memory(e) => Display::fmt(&e, f),
            Error::Wrapped(e, i) => write!(f, "{} - instruction {}", e, i),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(e) => Debug::fmt(&e, f),
            Error::Validate(e) => Debug::fmt(&e, f),
            Error::Execute(e) => Debug::fmt(&e, f),
            Error::Exit(e) => Debug::fmt(&e, f),
            Error::Memory(e) => Debug::fmt(&e, f),
            Error::Wrapped(e, i) => write!(f, "{:?} - instruction {}", e, i),
        }
    }
}

/// Wraps the Err in an error that contains the position of the instruction.
/// Otherwise, returns the result.
pub(crate) fn wrap<T>(result: Result<T, Error>, i: usize) -> Result<T, Error> {
    match result {
        Ok(val) => Ok(val),
        Err(err) => Err(err.wrap(i)),
    }
}
