use std::fmt::{Display, Formatter};

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
    pub fn value(&self) -> Result<u64, String> {
        match self {
            Operand::Constant(v) => Ok(v.clone()),
            Operand::Variable(_, v) => match v {
                Some(v) => Ok(*v),
                None => Err("Operand has no value".to_string()),
            },
            Operand::Label(_) => Err("Operand has no value".to_string()), // TODO get address of label?
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