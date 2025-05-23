use std::fmt::{Debug, Display};
use crate::{Error, Span};

/// A semantic result.
pub type SemResult<T> = Result<T, Error>;

/// A semantic error.
pub struct SemError {
    pub msg: String,
    pub span: Span
}

impl SemError {
    pub fn error<T>(msg: T, span: Span) -> Error
    where T: Into<String> {
        Error::Sem(Self { msg: msg.into(), span })
    }
}

impl Display for SemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Semantic Error: {} at {}", self.msg, self.span)
    }
}

impl Debug for SemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
