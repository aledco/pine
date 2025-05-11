use crate::env::Environment;
use crate::operand::*;
use crate::parse::{Parse, Line, Token, Literal};
use crate::inst::Instruction;
use std::fmt::Debug;
use std::ops::*;

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

// TODO currently cannot perform an operation on a float and int, such as `addf 1.1 1`.

#[inst(name = "add", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = wrapping_add, ty1 = i64, ty2 = i64)]
pub struct AddInst {}

#[inst(name = "addf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = add, ty1 = f64, ty2 = f64)]
pub struct AddfInst {}

#[inst(name = "sub", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = wrapping_sub, ty1 = i64, ty2 = i64)]
pub struct SubInst {}

#[inst(name = "subf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = sub, ty1 = f64, ty2 = f64)]
pub struct SubfInst {}

#[inst(name = "mul", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = wrapping_mul, ty1 = i64, ty2 = i64)]
pub struct MulInst {}

#[inst(name = "mulf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = mul, ty1 = f64, ty2 = f64)]
pub struct MulfInst {}

#[inst(name = "div", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = wrapping_div, ty1 = i64, ty2 = i64)]
pub struct DivInst {}

#[inst(name = "divf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = div, ty1 = f64, ty2 = f64)]
pub struct DivfInst {}

#[inst(name = "mod", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = wrapping_rem, ty1 = i64, ty2 = i64)]
pub struct ModInst {}

#[inst(name = "modf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = rem, ty1 = f64, ty2 = f64)]
pub struct ModfInst {}

#[inst(name = "pow", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = wrapping_pow, ty1 = i64, ty2 = u32)]
pub struct PowInst {}

#[inst(name = "powf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = powf, ty1 = f64, ty2 = f64)]
pub struct PowfInst {}

#[cfg(test)]
mod tests {
    use crate::ExecuteConfig;
    use crate::cast::*;
    use super::*;

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
}