use std::fmt::{Display, Formatter};
use std::ops::Add;

// #[derive(Debug, Clone, Copy)]
// pub enum Value {
//     Byte(u8),
//     Integer(i64),
//     Float(f64),
//     Address(usize)
// }

// impl Add for Value {
//     type Output = Value;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         match self {
//             Value::Byte(b) => {
//                 match rhs {
//                     Value::Byte(b) => Value::Byte(b + b),
//                     Value::Integer(_) => 0
//                     Value::Float(_) => {}
//                     Value::Address(_) => {}
//                 }
//             }
//             Value::Integer(_) => {}
//             Value::Float(_) => {}
//             Value::Address(_) => {}
//         }
//     }
// }

#[derive(Debug)]
pub enum Operand{
    Constant(u64),
    Variable(String, Option<u64>), // TODO will need a symbol table so that variables can share values
    Label(String),
}

impl Operand {
    pub fn value(&self) -> Option<u64> {
        match self {
            Operand::Constant(v) => Some(v.clone()),
            Operand::Variable(_, v) => v.clone(),
            Operand::Label(_) => None,
        }
    }
    
    pub fn set_value(&mut self, value: u64) {
        if let Operand::Variable(_, v) = self {
            *v = Some(value);
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Constant(c) => write!(f, "{}", c),
            Operand::Variable(n, _) => write!(f, "{}", n),
            Operand::Label(l) => write!(f, "{}", l),
        }
    }
}