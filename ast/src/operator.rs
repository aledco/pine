use std::fmt::{format, Display};
use crate::ast::PineType;
use crate::token::{TokenMatch, TokenType};
use strum::{EnumProperty, IntoEnumIterator};
use strum_macros::{EnumIter, EnumProperty, EnumString};

/// Represents a Pine operator
#[derive(Debug, PartialEq, Copy, Clone, EnumIter, EnumString, EnumProperty)]
pub enum Operator {
    #[strum(
        serialize = "==",
        props(Value = "==", IsUnary = false, IsBinary = true)
    )]
    Equals,
    #[strum(
        serialize = "!=",
        props(Value = "!=", IsUnary = false, IsBinary = true)
    )]
    NotEquals,
    #[strum(serialize = ">", props(Value = ">", IsUnary = false, IsBinary = true))]
    GreaterThan,
    #[strum(serialize = "<", props(Value = "<", IsUnary = false, IsBinary = true))]
    LessThan,
    #[strum(
        serialize = ">=",
        props(Value = ">=", IsUnary = false, IsBinary = true)
    )]
    GreaterThanOrEqual,
    #[strum(
        serialize = "<=",
        props(Value = "<=", IsUnary = false, IsBinary = true)
    )]
    LessThanOrEqual,
    #[strum(
        serialize = "and",
        props(Value = "and", IsUnary = false, IsBinary = true)
    )]
    And,
    #[strum(
        serialize = "or",
        props(Value = "or", IsUnary = false, IsBinary = true)
    )]
    Or,
    #[strum(
        serialize = "not",
        props(Value = "not", IsUnary = true, IsBinary = false)
    )]
    Not,
    #[strum(serialize = "+", props(Value = "+", IsUnary = false, IsBinary = true))]
    Add,
    #[strum(serialize = "-", props(Value = "-", IsUnary = true, IsBinary = true))]
    Subtract,
    #[strum(serialize = "*", props(Value = "*", IsUnary = false, IsBinary = true))]
    Multiply,
    #[strum(serialize = "/", props(Value = "/", IsUnary = false, IsBinary = true))]
    Divide,
    #[strum(
        serialize = "**",
        props(Value = "**", IsUnary = false, IsBinary = true)
    )]
    Power,
    #[strum(serialize = "%", props(Value = "%", IsUnary = false, IsBinary = true))]
    Modulo,
}

impl Operator {
    pub fn all_values() -> Vec<String> {
        Self::iter()
            .filter(|p| p.get_str("Value").is_some())
            .map(|p| p.get_str("Value").unwrap())
            .map(|s| String::from(s))
            .collect()
    }

    pub fn all_unary_ops() -> Vec<Self> {
        Self::iter().filter(|op| op.is_unary()).collect()
    }

    pub fn all_binary_ops() -> Vec<Self> {
        Self::iter().filter(|op| op.is_binary()).collect()
    }

    pub fn binary_ops_by_precedence(precedence: i32) -> Vec<Self> {
        Self::all_binary_ops()
            .into_iter()
            .filter(|op| op.precedence() == precedence)
            .collect()
    }

    pub fn max_length() -> usize {
        Self::all_values()
            .into_iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .len()
    }

    pub fn max_precedence() -> i32 {
        Self::iter()
            .max_by(|a, b| a.precedence().cmp(&b.precedence()))
            .unwrap()
            .precedence()
    }

    pub fn min_precedence() -> i32 {
        Self::iter()
            .min_by(|a, b| a.precedence().cmp(&b.precedence()))
            .unwrap()
            .precedence()
    }

    pub fn precedence(&self) -> i32 {
        match self {
            Operator::Equals => 4,
            Operator::NotEquals => 4,
            Operator::GreaterThan => 4,
            Operator::LessThan => 4,
            Operator::GreaterThanOrEqual => 4,
            Operator::LessThanOrEqual => 4,
            Operator::And => 6,
            Operator::Or => 7,
            Operator::Not => 5,
            Operator::Add => 3,
            Operator::Subtract => 3,
            Operator::Multiply => 2,
            Operator::Divide => 2,
            Operator::Power => 1,
            Operator::Modulo => 2,
        }
    }

    pub fn binary_pine_type(&self, lhs: PineType, rhs: PineType) -> Result<PineType, String> {
        if !self.is_binary() {
            return Err(format!("Operator `{}` is not binary", self));
        }

        match self {
            Operator::Equals
            | Operator::NotEquals
            | Operator::GreaterThan
            | Operator::LessThan
            | Operator::GreaterThanOrEqual
            | Operator::LessThanOrEqual => {
                // TODO also need to ensure operator is defined for type
                if lhs != rhs {
                    return Err(format!("Operands of `{}` must have the same type", self));
                }

                Ok(PineType::Bool)
            }
            Operator::And | Operator::Or => {
                if lhs != PineType::Bool || rhs != PineType::Bool {
                    return Err(format!("Operands of `{}` must have type bool", self));
                }

                Ok(PineType::Bool)
            }
            Operator::Add
            | Operator::Subtract
            | Operator::Multiply
            | Operator::Divide
            | Operator::Power
            | Operator::Modulo => {
                if lhs != rhs {
                    return Err(format!("Operands of `{}` must have the same type", self));
                }

                Ok(lhs)
            }
            _ => unimplemented!(),
        }
    }

    pub fn unary_pine_type(&self, t: PineType) -> Result<PineType, String> {
        if !self.is_unary() {
            return Err(format!("Operator `{}` is not unary", self));
        }

        match self {
            Operator::Not => {
                if t != PineType::Bool {
                    return Err(format!("Operand of `{}` must have type bool", self));
                }

                Ok(PineType::Bool)
            }
            Operator::Subtract => {
                // TODO ensure operator is defined for type
                Ok(t)
            }
            _ => unimplemented!(),
        }
    }

    pub fn is_unary(&self) -> bool {
        self.get_bool("IsUnary").unwrap()
    }

    pub fn is_binary(&self) -> bool {
        self.get_bool("IsBinary").unwrap()
    }
    
    pub fn as_str(&self) -> &str {
        self.get_str("Value").unwrap()
    }
}

impl TokenMatch for Operator {
    fn matches(&self, token_type: &TokenType) -> bool {
        match token_type {
            TokenType::Operator(o) => self == o,
            _ => false,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}