use std::fmt::{Display, Formatter};
use crate::env::Environment;
use crate::Error;
use crate::inst::{ValidateError, ExecuteError};

/// An operand of an instruction.
#[derive(Debug, Clone)]
pub enum Operand {
    Constant(u64),
    Variable(String),
    Label(String),
}

/// The operand format.
pub enum OperandFormat {
    /// The constant only operand format
    Constant,
    /// The variable only operand format
    Variable,
    /// The constant or variable operand format
    Value,
    /// The label only operand format
    Label
}

impl Operand {
    /// Gets the value of an operand.
    pub fn value(&self, env: &Environment) -> Result<u64, Error> {
        match self {
            Operand::Constant(v) => Ok(v.clone()),
            Operand::Variable(n) => match env.variables.get(n) {
                Some(v) => Ok(v.clone()),
                None => Err(ExecuteError::variable_does_not_exist(&n))
            },
            _ => Err(ExecuteError::operand_has_no_value()),
        }
    }

    /// Sets the value of an operand.
    pub fn set_value(&mut self, value: u64, env: &mut Environment) -> Result<(), Error> {
        match self {
            Operand::Variable(n) => {
                env.variables.insert(n.clone(), value);
                Ok(())
            }
            _ => Err(ExecuteError::operand_is_not_variable())
        }
    }

    /// Gets the variable name of an operand.
    pub fn var_name(&self) -> Result<String, Error>{
        match self {
            Operand::Variable(n) => Ok(n.clone()),
            _ => Err(ExecuteError::operand_is_not_variable()),
        }
    }

    /// Gets the label of an operand.
    pub fn label(&self) -> Result<String, Error>{
        match self {
            Operand::Label(l) => Ok(l.clone()),
            _ => Err(ExecuteError::operand_is_not_label()),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Constant(c) => write!(f, "{}", c),
            Operand::Variable(n) => write!(f, "{}", n),
            Operand::Label(l) => write!(f, "{}", l),
        }
    }
}

impl OperandFormat {
    /// Validates that an operand has the correct format.
    pub fn validate(&self, operand: &Operand) -> Result<(), Error> {
        match self {
            OperandFormat::Constant => {
                if matches!(operand, Operand::Constant(_)) {
                    Ok(())
                } else {
                    Err(ValidateError::operand_must_be_constant())
                }
            }
            OperandFormat::Variable => {
                if matches!(operand, Operand::Variable(_)) {
                    Ok(())
                } else {
                    Err(ValidateError::operand_must_be_variable())
                }
            }
            OperandFormat::Value => {
                if matches!(operand, Operand::Constant(_)) || matches!(operand, Operand::Variable(_)) {
                    Ok(())
                } else {
                    Err(ValidateError::operand_must_be_constant_or_variable())
                }
            }
            OperandFormat::Label => {
                if matches!(operand, Operand::Label(_)) {
                    Ok(())
                } else {
                    Err(ValidateError::operand_must_be_label())
                }
            }
        }
    }
}