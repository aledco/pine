use crate::env::Environment;
use crate::inst::*;
use crate::parse::{Line, Literal, Parse, Token};
use crate::cast::*;
use std::ops::Neg;

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

/// Moves a value into a variable.
#[inst(name = "move", operands = [OperandFormat::Variable, OperandFormat::Value])]
pub struct MoveInst {
    pub(crate) dest: Operand,
    pub(crate) src: Operand,
}

impl Instruction for MoveInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let val = self.src.value(env)?;
        self.dest.set_value(val, env)?;
        Ok(())
    }
}

/// Negates a signed integer.
#[inst(name = "neg", operands = [OperandFormat::Variable, OperandFormat::Value])]
pub struct NegInst {
    pub(crate) dest: Operand,
    pub(crate) src: Operand,
}

impl Instruction for NegInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let val = from_u64!(self.src.value(env)?; i64);
        let res = i64::neg(val);
        self.dest.set_value(to_u64!(res), env)?;
        Ok(())
    }
}

/// Negates a floating point number.
#[inst(name = "negf", operands = [OperandFormat::Variable, OperandFormat::Value])]
pub struct NegfInst {
    pub(crate) dest: Operand,
    pub(crate) src: Operand,
}

impl Instruction for NegfInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let val = from_u64!(self.src.value(env)?; f64);
        let res = f64::neg(val);
        self.dest.set_value(to_u64!(res), env)?;
        Ok(())
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

    #[test]
    #[should_panic]
    fn test_neg_validation() {
        let d = Operand::Constant(0);
        let s = Operand::Constant(0);
        let inst = NegInst::new(d, s);
        inst.validate().unwrap();
    }

    #[test]
    fn test_neg_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v in -32i64..32 {
            i += 1;

            let d = Operand::Variable(String::from("x"));
            let s = Operand::Constant(to_u64!(v));
            let mut inst = NegInst::new(d, s);

            inst.execute(&mut context).unwrap();
            inst.inc_inst_ptr(&mut context).unwrap();
            assert_eq!(
                from_u64!(inst.dest.value(&mut context).unwrap(); i64),
                i64::neg(v),
                "{} != -{}",
                inst.dest.value(&mut context).unwrap(),
                v
            );
            assert_eq!(context.inst_ptr, i);
        }
    }

    #[test]
    fn test_neg_display() {
        let d = Operand::Variable("x".to_string());
        let s = Operand::Constant(0);
        let inst = NegInst::new(d, s);
        let display = format!("{}", inst);
        assert_eq!(display, "neg x 0");
    }

    #[test]
    #[should_panic]
    fn test_negf_validation() {
        let d = Operand::Constant(0);
        let s = Operand::Constant(0);
        let inst = NegfInst::new(d, s);
        inst.validate().unwrap();
    }

    #[test]
    fn test_negf_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        let vals: Vec<f64> = vec![-100.14, -1.0, 0.0, 1.0, 100.14];
        for v in vals {
            i += 1;

            let d = Operand::Variable(String::from("x"));
            let s = Operand::Constant(to_u64!(v));
            let mut inst = NegfInst::new(d, s);

            inst.execute(&mut context).unwrap();
            inst.inc_inst_ptr(&mut context).unwrap();
            assert_eq!(
                from_u64!(inst.dest.value(&mut context).unwrap(); f64),
                f64::neg(v),
                "{} != -{}",
                inst.dest.value(&mut context).unwrap(),
                v
            );
            assert_eq!(context.inst_ptr, i);
        }
    }

    #[test]
    fn test_negf_display() {
        let d = Operand::Variable("x".to_string());
        let s = Operand::Constant(to_u64!(100.14_f64));
        let inst = NegfInst::new(d, s);
        let display = format!("{}", inst);
        let expected = format!("negf x {}", to_u64!(100.14_f64));
        assert_eq!(display, expected);
    }
}
