use crate::env::Environment;
use crate::inst::Instruction;
use crate::operand::*;
use crate::parse::{Line, Literal, Parse, Token};
use std::fmt::{Debug, Display, Formatter};
use std::ops::*;

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

// TODO use macro

#[inst(name = "move", operands = [OperandFormat::Variable, OperandFormat::Value])]
pub struct MoveInst {
    pub dest: Operand,
    pub src: Operand,
}

impl Instruction for MoveInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), String> {
        let val = self.src.value(env)?;
        self.dest.set_value(val, env);
        Ok(())
    }

    fn defined_var(&self) -> Option<Operand> {
        Some(self.dest.clone())
    }

    fn used_vars(&self) -> Vec<Operand> {
        let mut vars = vec![];
        if let Operand::Variable(_) = self.src {
            vars.push(self.src.clone());
        }

        vars
    }

    fn validate(&self) -> Result<(), String> {
        if !matches!(self.dest, Operand::Variable(_)) {
            Err("dest must be a label".to_string())
        } else if matches!(self.src, Operand::Label(_)) {
            Err("src must be a constant or variable".to_string())
        } else {
            Ok(())
        }
    }
}

impl Display for MoveInst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", Self::NAME, self.dest, self.src)
    }
}

#[cfg(test)]
mod tests {
    use crate::{from_u64, to_u64, ExecuteConfig};
    use super::*;

    #[test]
    #[should_panic]
    fn test_move_validation() {
        let d = Operand::Constant(0);
        let s = Operand::Constant(0);
        let inst = MoveInst::new(d, s);
        inst.validate().unwrap();
    }

    #[test]
    fn test_move_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v in -32i64..32 {
            i += 1;

            let d = Operand::Variable(String::from("x"));
            let s = Operand::Constant(to_u64!(v));
            let mut inst = MoveInst::new(d, s);

            inst.execute(&mut context).unwrap();
            inst.inc_inst_ptr(&mut context).unwrap();
            assert_eq!(
                from_u64!(inst.dest.value(&mut context).unwrap(); i64),
                v,
                "{} != {}",
                inst.dest.value(&mut context).unwrap(),
                v
            );
            assert_eq!(context.inst_ptr, i);
        }
    }

    #[test]
    fn test_move_display() {
        let d = Operand::Variable("x".to_string());
        let s = Operand::Constant(0);
        let inst = MoveInst::new(d, s);
        let display = format!("{}", inst);
        assert_eq!(display, "move x 0");
    }
}
