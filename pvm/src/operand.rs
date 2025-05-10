use std::fmt::{Display, Formatter};
use crate::env::Environment;

#[derive(Debug, Clone)]
pub enum Operand {
    Constant(u64),
    Variable(String),
    Label(String),
}

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
    pub fn value(&self, env: &Environment) -> Result<u64, String> {
        match self {
            Operand::Constant(v) => Ok(v.clone()),
            Operand::Variable(n) => match env.variables.get(n) {
                Some(v) => Ok(v.clone()),
                None => Err(format!("Variable {} does not exist", n))
            },
            Operand::Label(_) => Err("Operand has no value".to_string()),
        }
    }
    
    pub fn set_value(&mut self, value: u64, env: &mut Environment) {
        if let Operand::Variable(n) = self {
            env.variables.insert(n.clone(), value);
        }
    }

    pub fn var_name(&self) -> Result<String, String>{
        match self {
            Operand::Variable(n) => Ok(n.clone()),
            _ => Err("Operand is not a variable".to_string()),
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