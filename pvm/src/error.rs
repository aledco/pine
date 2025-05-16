use std::fmt::{Debug, Display};
use crate::parse::ParseError;
use crate::inst::{ValidateError, ExecuteError};
use crate::env::MemoryError;

#[derive(Clone)]
pub enum Error {
    ParseError(ParseError),
    ValidateError(ValidateError),
    ExecuteError(ExecuteError),
    MemoryError(MemoryError),
    WrappedError(Box<Error>, usize)
}

impl Error {
    pub(crate) fn wrap(&self, i: usize) -> Error {
        match self {
            Error::ValidateError(e) => Error::WrappedError(Box::new(self.clone()), i),
            Error::ExecuteError(e) => Error::WrappedError(Box::new(self.clone()), i),
            Error::MemoryError(e) => Error::WrappedError(Box::new(self.clone()), i),
            e => e.clone()
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(e) => Display::fmt(&e, f),
            Error::ValidateError(e) => Display::fmt(&e, f),
            Error::ExecuteError(e) => Display::fmt(&e, f),
            Error::MemoryError(e) => Display::fmt(&e, f),
            Error::WrappedError(e, i) => write!(f, "{} - instruction {}", e, i),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(e) => Debug::fmt(&e, f),
            Error::ValidateError(e) => Debug::fmt(&e, f),
            Error::ExecuteError(e) => Debug::fmt(&e, f),
            Error::MemoryError(e) => Debug::fmt(&e, f),
            Error::WrappedError(e, i) => write!(f, "{:?} - instruction {}", e, i),
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
