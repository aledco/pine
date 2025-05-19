use std::fmt::{Debug, Display, Formatter};
use crate::error::Error;

/// The validate error.
#[derive(Clone)]
pub struct ValidateError {
    pub msg: String,
}

impl ValidateError {
    pub(crate) fn error<T>(msg: T) -> Error
    where T: Into<String> {
        Error::ValidateError(Self {
            msg: msg.into(),
        })
    }
    
    pub(crate) fn label_already_defined(label: &str) -> Error {
        Self::error(format!("label {} already defined", label))
    }
    
    pub(crate) fn operand_must_be_constant() -> Error {
        Self::error("operand must be a constant")
    }
    
    pub(crate) fn operand_must_be_variable() -> Error {
        Self::error("operand must be a variable")
    }

    pub(crate) fn operand_must_be_constant_or_variable() -> Error {
        Self::error("operand must be a constant or variable")
    }

    pub(crate) fn operand_must_be_label() -> Error {
        Self::error("operand must be a label")
    }
}

impl Display for ValidateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validation Error: {}", self.msg)
    }
}

impl Debug for ValidateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validation Error: {}", self.msg)
    }
}

/// The execute error.
#[derive(Clone)]
pub struct ExecuteError {
    pub msg: String,
}

impl ExecuteError {
    pub(crate) fn error<T>(msg: T) -> Error
    where T: Into<String> {
        Error::ExecuteError(Self {
            msg: msg.into(),
        })
    }
    
    pub(crate) fn variable_does_not_exist(name: &str) -> Error {
        Self::error(format!("variable {} does not exist", name))  
    }
    
    pub(crate) fn label_does_not_exist(label: &str) -> Error {
        Self::error(format!("label {} does not exist", label))
    }
    
    pub(crate) fn operand_has_no_value() -> Error {
        Self::error("operand has no value")
    }

    pub(crate) fn operand_is_not_variable() -> Error {
        Self::error("operand is not a variable")
    }

    pub(crate) fn operand_is_not_label() -> Error {
        Self::error("operand is not a label")
    }
    
    pub(crate) fn arg_queue_is_empty() -> Error {
        Self::error("argument queue is empty")
    }

    pub(crate) fn ret_queue_is_empty() -> Error {
        Self::error("return value queue is empty")
    }
    
    pub(crate) fn ret_addr_stack_is_empty() -> Error {
        Self::error("return address stack is empty")
    }
    
    pub(crate) fn local_var_not_saved(name: &str) -> Error {
        Self::error(format!("local variable {} not saved", name))
    }
}

impl Display for ExecuteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime Error: {}", self.msg)
    }
}

impl Debug for ExecuteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime Error: {}", self.msg)
    }
}

#[derive(Clone)]
pub struct ExitError {
    pub exit_code: i32,
}

impl ExitError {
    pub(crate) fn exit(code: i32) -> Error {
        Error::ExitError(Self {
            exit_code: code,
        })
    }
}

impl Display for ExitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "process exited with code {}", self.exit_code)
    }
}

impl Debug for ExitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "process exited with code {}", self.exit_code)
    }
}
