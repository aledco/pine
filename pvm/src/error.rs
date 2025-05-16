use std::fmt::Debug;
use crate::parse::ParseError;
use crate::inst::{ValidateError, ExecuteError};
use crate::env::MemoryError;

pub enum Error {
    ParseError(ParseError),
    ValidateError(ValidateError),
    ExecuteError(ExecuteError),
    MemoryError(MemoryError),
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { // TODO get better error messages, figure out how to pass the line number (don't have access to line when building insts from API)
            Error::ParseError(e) => e.fmt(f),
            Error::ValidateError(e) => e.fmt(f),
            Error::ExecuteError(e) => e.fmt(f),
            Error::MemoryError(e) => e.fmt(f),
        }
    }
}