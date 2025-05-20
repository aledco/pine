use std::fmt::{Debug, Display};
use crate::Span;

pub type ParseResult<T> = Result<T, ParseError>;

pub struct ParseError {
    pub msg: String,
    pub span: Span
}

impl ParseError {
    pub fn new<T>(msg: T, span: Span) -> Self
    where T: Into<String> {
        Self { msg: msg.into(), span }
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
