use std::fmt::Debug;
use crate::parse::ParseError;

pub enum Error {
    ParseError(ParseError),
    MemoryError,
    
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseError(e) => e.fmt(f),
            Error::MemoryError => unimplemented!(),
        }
    }
}