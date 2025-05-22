use std::fmt::{Debug, Display, Formatter};
use crate::error::Error;

/// The parse error.
#[derive(Clone)]
pub struct ParseError {
    pub msg: String,
    pub line: usize,
}

impl ParseError {
    pub(crate) fn error<T>(msg: T, line: usize) -> Error
    where T: Into<String> {
        Error::Parse(Self {
            msg: msg.into(),
            line,
        })
    }
    
    pub(crate) fn inst_not_recognized(inst: &str, line: usize) -> Error {
        Self::error(format!("instruction {} not recognized", inst), line)
    }

    pub(crate) fn invalid_token(line: usize) -> Error {
        Self::error("invalid token", line)
    }
    
    pub(crate) fn invalid_number_of_operands(inst: &str, expected: usize, actual: usize, line: usize) -> Error {
        Self::error(format!("invalid number of operands for {}; expected {} but got {}", inst, expected, actual), line)
    }

    pub(crate) fn invalid_operand_format(line: usize) -> Error {
        Self::error("invalid operand format", line)
    }
    
    pub(crate) fn invalid_operand(line: usize) -> Error {
        Self::error("invalid operand", line)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse Error at line {}: {}", self.line, self.msg)
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse Error at line {}: {}", self.line, self.msg)
    }
}
