// use crate::env::Environment;
// use crate::inst::{Instruction, PrintcInst};
// use crate::operand::*;
// use crate::parse::{Line, Literal, Parse, Token};
// use std::fmt::Debug;
// use pvm_proc_macros::inst;
// 
// #[inst(name = "jump", operands = [OperandFormat::Value])]
// pub struct JumpInst {
//     pub src: Operand,
// }
// 
// impl Instruction for JumpInst {
//     fn execute(&mut self, env: &mut Environment) -> Result<(), String> {
//         let value = crate::cast::from_u64!(self.src.value(env)?; u8);
//         let res = {
//             let c = char::try_from(value).unwrap_or('?');
//             write!(env.stdout.borrow_mut(), "{}", c)
//         };
//         match res {
//             Ok(_) => Ok(()),
//             Err(e) => Err(format!("{}", e)),
//         }
//     }
// 
//     fn used_vars(&self) -> Vec<Operand> {
//         if let Operand::Variable(_) = self.src {
//             return vec![self.src.clone()];
//         }
// 
//         vec![]
//     }
// 
//     fn validate(&self) -> Result<(), String> {
//         if matches!(self.src, Operand::Label(_)) {
//             Err("src must be a variable or constant".to_string())
//         } else {
//             Ok(())
//         }
//     }
// }