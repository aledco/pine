use std::fmt::{Debug, Display, Formatter};
use crate::error::Error;

/// The memory error.
#[derive(Clone)]
pub struct MemoryError {
    pub msg: String,
}

impl MemoryError {
    pub(crate) fn error<T>(msg: T) -> Error
    where T: Into<String> {
        Error::Memory(Self {
            msg: msg.into(),
        })
    }

    pub(crate) fn cannot_allocate_zero_bytes() -> Error {
        Self::error("cannot allocate zero bytes")
    }

    pub(crate) fn out_of_memory() -> Error {
        Self::error("out of memory")
    }

    pub(crate) fn address_out_of_bounds() -> Error {
        Self::error("address out of bounds")
    }

    pub(crate) fn invalid_address() -> Error {
        Self::error("invalid address")
    }
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Memory Error: {}", self.msg)
    }
}

impl Debug for MemoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Memory Error: {}", self.msg)
    }
}
