use std::fmt::{Display, Formatter};
use crate::env::Environment;
use crate::operand::*;

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

pub trait Instruction {
    fn execute(&mut self, context: &mut Environment) -> Result<(), String>;

    fn inc_inst_ptr(&self, context: &mut Environment) -> Result<(), String> {
        context.inst_ptr += 1;
        Ok(())
    }
}

// TODO types of instructions: arithmetic, data, branch

#[arithmetic(2)]
#[bin_op(+)]
pub struct AddInst {}

impl Display for AddInst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {} + {}", self.dest, self.src1, self.src2)
    }
}

#[arithmetic(2)]
#[bin_op(-)]
pub struct SubInst {}

#[arithmetic(2)]
#[bin_op(*)]
pub struct MulInst {}

#[arithmetic(2)]
#[bin_op(/)]
pub struct DivInst {}

#[arithmetic(2)]
#[bin_op(%)]
pub struct ModInst {}


#[arithmetic(2)]
pub struct PowInst {}

impl Instruction for PowInst {
    fn execute(&mut self, context: &mut Environment) -> Result<(), String> {
        let val1 = self.src1.value()?;
        let val2 = self.src2.value()?;
        self.dest.set_value(val1.pow(val2 as u32)); // TODO does cast to u32 make sense here?
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[should_panic]
    fn test_add_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let _ = AddInst::new(d, s1, s2);
    }

    #[test]
    fn test_add_inst() {
        let mut i = 0;
        let mut context = Environment::new(32);
        for v1 in 0..256 {
            for v2 in 0..256 {
                i += 1;

                let d = Operand::Variable(String::from("x"), None);
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = AddInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(inst.dest.value().unwrap(), v1 + v2, "{} != {} + {}", inst.dest.value().unwrap(), v1, v2);
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_sub_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let _ = SubInst::new(d, s1, s2);
    }

    #[test]
    fn test_sub_inst() {
        let mut i = 0;
        let mut context = Environment::new(32);
        for v1 in 0..256 {
            for v2 in 0..256 {
                i += 1;

                let d = Operand::Variable(String::from("x"), None);
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = SubInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(inst.dest.value().unwrap(), v1 - v2, "{} != {} + {}", inst.dest.value().unwrap(), v1, v2);
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_mul_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let _ = MulInst::new(d, s1, s2);
    }

    #[test]
    fn test_mul_inst() {
        let d = Operand::Variable(String::from("x"), None);
        let s1 = Operand::Constant(4);
        let s2 = Operand::Constant(3);
        let mut inst = MulInst::new(d, s1, s2);
        let mut context = Environment::new(32);
        inst.execute(&mut context).unwrap();
        inst.inc_inst_ptr(&mut context).unwrap();
        assert_eq!(inst.dest.value(), Ok(12));
        assert_eq!(context.inst_ptr, 1);
    }

    #[test]
    #[should_panic]
    fn test_div_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let _ = DivInst::new(d, s1, s2);
    }

    #[test]
    fn test_div_inst() {
        let d = Operand::Variable(String::from("x"), None);
        let s1 = Operand::Constant(5);
        let s2 = Operand::Constant(2);
        let mut inst = DivInst::new(d, s1, s2);
        let mut context = Environment::new(32);
        inst.execute(&mut context).unwrap();
        inst.inc_inst_ptr(&mut context).unwrap();
        assert_eq!(inst.dest.value(), Ok(2));
        assert_eq!(context.inst_ptr, 1);
    }

    #[test]
    #[should_panic]
    fn test_mod_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let _ = ModInst::new(d, s1, s2);
    }

    #[test]
    fn test_mod_inst() {
        let d = Operand::Variable(String::from("x"), None);
        let s1 = Operand::Constant(8);
        let s2 = Operand::Constant(2);
        let mut inst = ModInst::new(d, s1, s2);
        let mut context = Environment::new(32);
        inst.execute(&mut context).unwrap();
        inst.inc_inst_ptr(&mut context).unwrap();
        assert_eq!(inst.dest.value(), Ok(0));
        assert_eq!(context.inst_ptr, 1);
    }

    #[test]
    #[should_panic]
    fn test_pow_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let _ = ModInst::new(d, s1, s2);
    }

    #[test]
    fn test_pow_inst() {
        let d = Operand::Variable(String::from("x"), None);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let mut inst = PowInst::new(d, s1, s2);
        let mut context = Environment::new(32);
        inst.execute(&mut context).unwrap();
        inst.inc_inst_ptr(&mut context).unwrap();
        assert_eq!(inst.dest.value(), Ok(8));
        assert_eq!(context.inst_ptr, 1);
    }
}