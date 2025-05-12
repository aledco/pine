use crate::env::Environment;
use crate::inst::Instruction;
use crate::operand::*;
use crate::parse::{Line, Literal, Parse, Token};
use std::fmt::Debug;
use std::ops::*;

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

// TODO currently cannot perform an operation on a float and int, such as `addf 1.1 1`.

#[inst(name = "add", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = i64::wrapping_add, ty1 = i64, ty2 = i64)]
pub struct AddInst {}

#[inst(name = "addf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = f64::add, ty1 = f64, ty2 = f64)]
pub struct AddfInst {}

#[inst(name = "sub", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = i64::wrapping_sub, ty1 = i64, ty2 = i64)]
pub struct SubInst {}

#[inst(name = "subf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = f64::sub, ty1 = f64, ty2 = f64)]
pub struct SubfInst {}

#[inst(name = "mul", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = i64::wrapping_mul, ty1 = i64, ty2 = i64)]
pub struct MulInst {}

#[inst(name = "mulf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = f64::mul, ty1 = f64, ty2 = f64)]
pub struct MulfInst {}

#[inst(name = "div", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = i64::wrapping_div, ty1 = i64, ty2 = i64)]
pub struct DivInst {}

#[inst(name = "divf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = f64::div, ty1 = f64, ty2 = f64)]
pub struct DivfInst {}

#[inst(name = "mod", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = i64::wrapping_rem, ty1 = i64, ty2 = i64)]
pub struct ModInst {}

#[inst(name = "modf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = f64::rem, ty1 = f64, ty2 = f64)]
pub struct ModfInst {}

#[inst(name = "pow", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = i64::wrapping_pow, ty1 = i64, ty2 = u32)]
pub struct PowInst {}

#[inst(name = "powf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = f64::powf, ty1 = f64, ty2 = f64)]
pub struct PowfInst {}

fn eq(v1: u64, v2: u64) -> u64 {
    (v1 == v2) as u64
}
fn neq(v1: u64, v2: u64) -> u64 {
    (v1 != v2) as u64
}
fn lt(v1: u64, v2: u64) -> u64 {
    (v1 < v2) as u64
}
fn lte(v1: u64, v2: u64) -> u64 {
    (v1 <= v2) as u64
}
fn gt(v1: u64, v2: u64) -> u64 {
    (v1 > v2) as u64
}
fn gte(v1: u64, v2: u64) -> u64 {
    (v1 >= v2) as u64
}

#[inst(name = "eq", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = eq, ty1 = u64, ty2 = u64)]
pub struct EqInst {}

#[inst(name = "neq", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = neq, ty1 = u64, ty2 = u64)]
pub struct NeqInst {}

#[inst(name = "lt", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = lt, ty1 = u64, ty2 = u64)]
pub struct LtInst {}

#[inst(name = "lte", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = lte, ty1 = u64, ty2 = u64)]
pub struct LteInst {}

#[inst(name = "gt", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = gt, ty1 = u64, ty2 = u64)]
pub struct GtInst {}

#[inst(name = "gte", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = gte, ty1 = u64, ty2 = u64)]
pub struct GteInst {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cast::*;
    use crate::ExecuteConfig;

    #[test]
    #[should_panic]
    fn test_add_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = AddInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_add_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(v1));
                let s2 = Operand::Constant(to_u64!(v2));
                let mut inst = AddInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); i64),
                    v1.wrapping_add(v2),
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
        assert_eq!(display, "add x 2 3");
    }

    #[test]
    #[should_panic]
    fn test_addf_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(100.14_f64));
        let inst = AddfInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_addf_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);

        let vals: Vec<f64> = vec![-1000.5 * 1000.4, -100.14, 0.0, 100.14, 1000.5 * 1000.4];
        for v1 in &vals {
            for v2 in &vals {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(*v1));
                let s2 = Operand::Constant(to_u64!(*v2));
                let mut inst = AddfInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); f64),
                    v1.add(v2),
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
    fn test_addf_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(0.02_f64));
        let inst = AddfInst::new(d, s1, s2);
        let display = format!("{}", inst);
        let expected = format!("addf x {} {}", to_u64!(100.14_f64), to_u64!(0.02_f64));
        assert_eq!(display, expected);
    }

    #[test]
    #[should_panic]
    fn test_sub_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = SubInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_sub_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(v1));
                let s2 = Operand::Constant(to_u64!(v2));
                let mut inst = SubInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); i64),
                    v1.wrapping_sub(v2),
                    "{} != {} - {}",
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
        assert_eq!(display, "sub x 2 3");
    }

    #[test]
    #[should_panic]
    fn test_subf_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(100.14_f64));
        let inst = SubfInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_subf_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);

        let vals: Vec<f64> = vec![-1000.5 * 1000.4, -100.14, 0.0, 100.14, 1000.5 * 1000.4];
        for v1 in &vals {
            for v2 in &vals {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(*v1));
                let s2 = Operand::Constant(to_u64!(*v2));
                let mut inst = SubfInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); f64),
                    v1.sub(v2),
                    "{} != {} - {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_subf_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(0.02_f64));
        let inst = SubfInst::new(d, s1, s2);
        let display = format!("{}", inst);
        let expected = format!("subf x {} {}", to_u64!(100.14_f64), to_u64!(0.02_f64));
        assert_eq!(display, expected);
    }

    #[test]
    #[should_panic]
    fn test_mul_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = MulInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_mul_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(v1));
                let s2 = Operand::Constant(to_u64!(v2));
                let mut inst = MulInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); i64),
                    v1.wrapping_mul(v2),
                    "{} != {} * {}",
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
        assert_eq!(display, "mul x 2 3");
    }

    #[test]
    #[should_panic]
    fn test_mulf_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(100.14_f64));
        let inst = MulfInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_mulf_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);

        let vals: Vec<f64> = vec![-1000.5 * 1000.4, -100.14, 0.0, 100.14, 1000.5 * 1000.4];
        for v1 in &vals {
            for v2 in &vals {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(*v1));
                let s2 = Operand::Constant(to_u64!(*v2));
                let mut inst = MulfInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); f64),
                    v1.mul(v2),
                    "{} != {} * {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_mulf_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(0.02_f64));
        let inst = MulfInst::new(d, s1, s2);
        let display = format!("{}", inst);
        let expected = format!("mulf x {} {}", to_u64!(100.14_f64), to_u64!(0.02_f64));
        assert_eq!(display, expected);
    }

    #[test]
    #[should_panic]
    fn test_div_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = DivInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_div_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                if v2 == 0 {
                    continue;
                }

                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(v1));
                let s2 = Operand::Constant(to_u64!(v2));
                let mut inst = DivInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); i64),
                    v1.wrapping_div(v2),
                    "{} != {} / {}",
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
        assert_eq!(display, "div x 2 3");
    }

    #[test]
    #[should_panic]
    fn test_divf_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(100.14_f64));
        let inst = DivfInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_divf_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        let vals: Vec<f64> = vec![-1000.5 * 1000.4, -100.14, 0.0, 100.14, 1000.5 * 1000.4];
        for v1 in &vals {
            for v2 in &vals {
                if *v2 == 0_f64 {
                    continue;
                }

                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(*v1));
                let s2 = Operand::Constant(to_u64!(*v2));
                let mut inst = DivfInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); f64),
                    v1.div(v2),
                    "{} != {} / {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_divf_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(0.02_f64));
        let inst = DivfInst::new(d, s1, s2);
        let display = format!("{}", inst);
        let expected = format!("divf x {} {}", to_u64!(100.14_f64), to_u64!(0.02_f64));
        assert_eq!(display, expected);
    }

    #[test]
    #[should_panic]
    fn test_mod_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = ModInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_mod_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                if v2 == 0 {
                    continue;
                }

                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(v1));
                let s2 = Operand::Constant(to_u64!(v2));
                let mut inst = ModInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); i64),
                    v1.wrapping_rem(v2),
                    "{} != {} % {}",
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
        assert_eq!(display, "mod x 2 3");
    }

    #[test]
    #[should_panic]
    fn test_modf_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(100.14_f64));
        let inst = ModfInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_modf_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);

        let vals: Vec<f64> = vec![-1000.5 * 1000.4, -100.14, 0.0, 100.14, 1000.5 * 1000.4];
        for v1 in &vals {
            for v2 in &vals {
                if *v2 == 0_f64 {
                    continue;
                }

                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(*v1));
                let s2 = Operand::Constant(to_u64!(*v2));
                let mut inst = ModfInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); f64),
                    v1.rem(v2),
                    "{} != {} % {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_modf_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(0.02_f64));
        let inst = ModfInst::new(d, s1, s2);
        let display = format!("{}", inst);
        let expected = format!("modf x {} {}", to_u64!(100.14_f64), to_u64!(0.02_f64));
        assert_eq!(display, expected);
    }

    #[test]
    #[should_panic]
    fn test_pow_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = PowInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_pow_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in -32i64..32 {
            for v2 in 0u32..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(v1));
                let s2 = Operand::Constant(to_u64!(v2));
                let mut inst = PowInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); i64),
                    v1.wrapping_pow(v2),
                    "{} != {} ^ {}",
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
        assert_eq!(display, "pow x 2 3");
    }

    #[test]
    #[should_panic]
    fn test_powf_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(100.14_f64));
        let inst = PowfInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_powf_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);

        let vals: Vec<f64> = vec![0.0, 2.5, 100.14];
        for v1 in &vals {
            for v2 in &vals {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(*v1));
                let s2 = Operand::Constant(to_u64!(*v2));
                let mut inst = PowfInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); f64),
                    v1.powf(*v2),
                    "{} != {} ^ {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_powf_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(0.02_f64));
        let inst = PowfInst::new(d, s1, s2);
        let display = format!("{}", inst);
        let expected = format!("powf x {} {}", to_u64!(100.14_f64), to_u64!(0.02_f64));
        assert_eq!(display, expected);
    }

    #[test]
    fn test_eq_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(v1));
                let s2 = Operand::Constant(to_u64!(v2));
                let mut inst = EqInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    eq(to_u64!(v1), to_u64!(v2)),
                    "{} != {} == {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_eq_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = EqInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "eq x 2 3");
    }

    #[test]
    fn test_neq_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(v1));
                let s2 = Operand::Constant(to_u64!(v2));
                let mut inst = NeqInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    neq(to_u64!(v1), to_u64!(v2)),
                    "{} != {} != {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_neq_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = NeqInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "neq x 2 3");
    }

    #[test]
    fn test_lt_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(v1));
                let s2 = Operand::Constant(to_u64!(v2));
                let mut inst = LtInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    lt(to_u64!(v1), to_u64!(v2)),
                    "{} != {} < {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_lt_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = LtInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "lt x 2 3");
    }

    #[test]
    fn test_lte_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(v1));
                let s2 = Operand::Constant(to_u64!(v2));
                let mut inst = LteInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    lte(to_u64!(v1), to_u64!(v2)),
                    "{} != {} <= {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_lte_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = LteInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "lte x 2 3");
    }

    #[test]
    fn test_gt_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(v1));
                let s2 = Operand::Constant(to_u64!(v2));
                let mut inst = GtInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    gt(to_u64!(v1), to_u64!(v2)),
                    "{} != {} > {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_gt_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = GtInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "gt x 2 3");
    }

    #[test]
    fn test_gte_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in -32i64..32 {
            for v2 in -32i64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(v1));
                let s2 = Operand::Constant(to_u64!(v2));
                let mut inst = GteInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    gte(to_u64!(v1), to_u64!(v2)),
                    "{} != {} >= {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_gte_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = GteInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "gte x 2 3");
    }
}
