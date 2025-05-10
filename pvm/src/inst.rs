use crate::env::Environment;
use crate::operand::*;
use std::fmt::{Display, Formatter};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

pub trait Instruction {
    fn execute(&mut self, env: &mut Environment) -> Result<(), String>;

    fn inc_inst_ptr(&self, env: &mut Environment) -> Result<(), String> {
        env.inst_ptr += 1;
        Ok(())
    }

    fn defined_label(&self) -> Option<String> {
        None
    }
    fn defined_var(&self) -> Option<Operand> {
        None
    }
    fn used_vars(&self) -> Vec<Operand> {
        Vec::new()
    }
}

// TODO types of instructions: arithmetic, data, branch

// BEGIN BINARY OP INSTRUCTIONS
#[inst(name = "add", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[arithmetic]
#[bin_op(op = wrapping_add, ty1 = i64, ty2 = i64)] // TODO use bin_op_helper instead of arithmetic?
pub struct AddInst {}

#[inst(name = "sub", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[arithmetic]
#[bin_op(op = wrapping_sub, ty1 = i64, ty2 = i64)]
pub struct SubInst {}

#[inst(name = "mul", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[arithmetic]
#[bin_op(op = wrapping_mul, ty1 = i64, ty2 = i64)]
pub struct MulInst {}

#[inst(name = "div", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[arithmetic]
#[bin_op(op = wrapping_div, ty1 = i64, ty2 = i64)]
pub struct DivInst {}

#[inst(name = "mod", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[arithmetic]
#[bin_op(op = wrapping_rem, ty1 = i64, ty2 = i64)]
pub struct ModInst {}

#[inst(name = "pow", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[arithmetic]
#[bin_op(op = wrapping_pow, ty1 = i64, ty2 = u32)]
pub struct PowInst {}
// END BINARY OP INSTRUCTIONS

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
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1 as u64);
                let s2 = Operand::Constant(v2 as u64);
                let mut inst = AddInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    v1.wrapping_add(v2) as u64,
                    "{} != {} + {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_add_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = AddInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "add x, 2, 3");
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
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1 as u64);
                let s2 = Operand::Constant(v2 as u64);
                let mut inst = SubInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    v1.wrapping_sub(v2) as u64,
                    "{} != {} + {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_sub_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = SubInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "sub x, 2, 3");
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
        let mut i = 0;
        let mut context = Environment::new(32);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1 as u64);
                let s2 = Operand::Constant(v2 as u64);
                let mut inst = MulInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    v1.wrapping_mul(v2) as u64,
                    "{} != {} + {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_mul_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = MulInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "mul x, 2, 3");
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
        let mut i = 0;
        let mut context = Environment::new(32);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                if v2 == 0 {
                    continue;
                }

                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1 as u64);
                let s2 = Operand::Constant(v2 as u64);
                let mut inst = DivInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    v1.wrapping_div(v2) as u64,
                    "{} != {} + {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_div_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = DivInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "div x, 2, 3");
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
        let mut i = 0;
        let mut context = Environment::new(32);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                if v2 == 0 {
                    continue;
                }

                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1 as u64);
                let s2 = Operand::Constant(v2 as u64);
                let mut inst = ModInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    v1.wrapping_rem(v2) as u64,
                    "{} != {} + {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_mod_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = ModInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "mod x, 2, 3");
    }

    #[test]
    #[should_panic]
    fn test_pow_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let _ = PowInst::new(d, s1, s2);
    }

    #[test]
    fn test_pow_inst() {
        let mut i = 0;
        let mut context = Environment::new(32);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1 as u64);
                let s2 = Operand::Constant(v2 as u64);
                let mut inst = PowInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                let (r, _) = v1.overflowing_pow(v2 as u32);
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    v1.wrapping_pow(v2 as u32) as u64,
                    "{} != {} + {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_pow_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = PowInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "pow x, 2, 3");
    }
}
