use std::fmt::{Debug, Display};
use crate::Span;
use crate::sem::SemError;

pub enum Error {
    Parse(ParseError),
    Sem(SemError)
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(e) => Display::fmt(&e, f),
            Error::Sem(e) => Display::fmt(&e, f),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Parse(e) => Debug::fmt(&e, f),
            Error::Sem(e) => Debug::fmt(&e, f),
        }
    }
}

pub(crate) type ParseResult<T> = Result<T, Error>;

pub struct ParseError {
    pub msg: String,
    pub span: Span
}

impl ParseError {
    pub fn error<T>(msg: T, span: Span) -> Error
    where T: Into<String> {
        Error::Parse(Self{ msg: msg.into(), span })
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse Error: {} at {}", self.msg, self.span)
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
