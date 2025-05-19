use crate::env::Environment;
use crate::inst::*;
use crate::parse::{Line, Literal, Parse, Token};
use std::ops::*;

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

/// Adds two signed integers.
#[inst(name = "add", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = i64::wrapping_add, ty1 = i64, ty2 = i64)]
pub struct AddInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Adds two unsigned integers.
#[inst(name = "addu", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = u64::wrapping_add, ty1 = u64, ty2 = u64)]
pub struct AdduInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Adds two floating point numbers.
#[inst(name = "addf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = f64::add, ty1 = f64, ty2 = f64)]
pub struct AddfInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Subtracts two signed integers.
#[inst(name = "sub", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = i64::wrapping_sub, ty1 = i64, ty2 = i64)]
pub struct SubInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Subtracts two unsigned integers.
#[inst(name = "subu", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = u64::wrapping_sub, ty1 = u64, ty2 = u64)]
pub struct SubuInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Subtracts two floating point numbers.
#[inst(name = "subf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = f64::sub, ty1 = f64, ty2 = f64)]
pub struct SubfInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Multiplies two signed integers.
#[inst(name = "mul", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = i64::wrapping_mul, ty1 = i64, ty2 = i64)]
pub struct MulInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Multiplies two unsigned integers.
#[inst(name = "mulu", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = u64::wrapping_mul, ty1 = u64, ty2 = u64)]
pub struct MuluInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Multiplies two floating point numbers.
#[inst(name = "mulf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = f64::mul, ty1 = f64, ty2 = f64)]
pub struct MulfInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Divides two signed integers.
#[inst(name = "div", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = i64::wrapping_div, ty1 = i64, ty2 = i64)]
pub struct DivInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Divides two unsigned integers.
#[inst(name = "divu", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = u64::wrapping_div, ty1 = u64, ty2 = u64)]
pub struct DivuInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Divides two floating point numbers.
#[inst(name = "divf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = f64::div, ty1 = f64, ty2 = f64)]
pub struct DivfInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Takes the modulo of two signed integers.
#[inst(name = "mod", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = i64::wrapping_rem, ty1 = i64, ty2 = i64)]
pub struct ModInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Takes the modulo of two unsigned integers.
#[inst(name = "modu", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = u64::wrapping_rem, ty1 = u64, ty2 = u64)]
pub struct ModuInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Takes the modulo of two floating point numbers.
#[inst(name = "modf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = f64::rem, ty1 = f64, ty2 = f64)]
pub struct ModfInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Takes the power of a signed integer.
#[inst(name = "pow", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = i64::wrapping_pow, ty1 = i64, ty2 = u32)]
pub struct PowInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Takes the power of an unsigned integer.
#[inst(name = "powu", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = u64::wrapping_pow, ty1 = u64, ty2 = u32)]
pub struct PowuInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Takes the power of a floating point number.
#[inst(name = "powf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = f64::powf, ty1 = f64, ty2 = f64)]
pub struct PowfInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

fn eq(v1: i64, v2: i64) -> u8 {
    (v1 == v2) as u8
}
fn equ(v1: u64, v2: u64) -> u8 {
    (v1 == v2) as u8
}
fn eqf(v1: f64, v2: f64) -> u8 {
    (v1 == v2) as u8
}
fn neq(v1: i64, v2: i64) -> u8 {
    (v1 != v2) as u8
}
fn nequ(v1: u64, v2: u64) -> u8 {
    (v1 != v2) as u8
}
fn neqf(v1: f64, v2: f64) -> u8 {
    (v1 != v2) as u8
}
fn lt(v1: i64, v2: i64) -> u8 {
    (v1 < v2) as u8
}
fn ltu(v1: u64, v2: u64) -> u8 {
    (v1 < v2) as u8
}
fn ltf(v1: f64, v2: f64) -> u8 {
    (v1 < v2) as u8
}
fn lte(v1: i64, v2: i64) -> u8 {
    (v1 <= v2) as u8
}
fn lteu(v1: u64, v2: u64) -> u8 {
    (v1 <= v2) as u8
}
fn ltef(v1: f64, v2: f64) -> u8 {
    (v1 <= v2) as u8
}
fn gt(v1: i64, v2: i64) -> u8 {
    (v1 > v2) as u8
}
fn gtu(v1: u64, v2: u64) -> u8 {
    (v1 > v2) as u8
}
fn gtf(v1: f64, v2: f64) -> u8 {
    (v1 > v2) as u8
}
fn gte(v1: i64, v2: i64) -> u8 {
    (v1 >= v2) as u8
}
fn gteu(v1: u64, v2: u64) -> u8 {
    (v1 >= v2) as u8
}
fn gtef(v1: f64, v2: f64) -> u8 {
    (v1 >= v2) as u8
}

/// Determines if one signed integer is equal to another.
#[inst(name = "eq", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = eq, ty1 = i64, ty2 = i64)]
pub struct EqInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Determines if one unsigned integer is equal to another.
#[inst(name = "equ", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = equ, ty1 = u64, ty2 = u64)]
pub struct EquInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Determines if one floating point number is equal to another.
#[inst(name = "eqf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = eqf, ty1 = f64, ty2 = f64)]
pub struct EqfInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Determines if one signed integer is not equal to another.
#[inst(name = "neq", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = neq, ty1 = i64, ty2 = i64)]
pub struct NeqInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Determines if one unsigned integer is not equal to another.
#[inst(name = "nequ", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = nequ, ty1 = u64, ty2 = u64)]
pub struct NequInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Determines if one floating point number is not equal to another.
#[inst(name = "neqf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = neqf, ty1 = f64, ty2 = f64)]
pub struct NeqfInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Determines if one signed integer is less than another.
#[inst(name = "lt", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = lt, ty1 = i64, ty2 = i64)]
pub struct LtInst {
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

/// Determines if one unsigned integer is less than another.
#[inst(name = "ltu", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = ltu, ty1 = u64, ty2 = u64)]
pub struct LtuInst {
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

/// Determines if one floating point number is less than another.
#[inst(name = "ltf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = ltf, ty1 = f64, ty2 = f64)]
pub struct LtfInst {
    pub dest: Operand,
    pub src1: Operand,
    pub src2: Operand,
}

/// Determines if one signed integer is less than or equal to another.
#[inst(name = "lte", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = lte, ty1 = i64, ty2 = i64)]
pub struct LteInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Determines if one unsigned integer is less than or equal to another.
#[inst(name = "lteu", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = lteu, ty1 = u64, ty2 = u64)]
pub struct LteuInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Determines if one floating point number is less than or equal to another.
#[inst(name = "ltef", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = ltef, ty1 = f64, ty2 = f64)]
pub struct LtefInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Determines if one signed integer is greater than another.
#[inst(name = "gt", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = gt, ty1 = i64, ty2 = i64)]
pub struct GtInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Determines if one unsigned integer is greater than another.
#[inst(name = "gtu", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = gtu, ty1 = u64, ty2 = u64)]
pub struct GtuInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Determines if one floating point number is greater than another.
#[inst(name = "gtf", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = gtf, ty1 = f64, ty2 = f64)]
pub struct GtfInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Determines if one signed integer is greater than or equal to another.
#[inst(name = "gte", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = gte, ty1 = i64, ty2 = i64)]
pub struct GteInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Determines if one unsigned integer is greater than or equal to another.
#[inst(name = "gteu", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = gteu, ty1 = u64, ty2 = u64)]
pub struct GteuInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Determines if one floating point number is greater than or equal to another.
#[inst(name = "gtef", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = gtef, ty1 = f64, ty2 = f64)]
pub struct GtefInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Applies the and bitwise operation.
#[inst(name = "and", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = u64::bitand, ty1 = u64, ty2 = u64)]
pub struct AndInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Applies the or bitwise operation.
#[inst(name = "or", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = u64::bitor, ty1 = u64, ty2 = u64)]
pub struct OrInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Applies the exclusive or bitwise operation.
#[inst(name = "xor", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = u64::bitxor, ty1 = u64, ty2 = u64)]
pub struct XorInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Applies the shift left bitwise operation.
#[inst(name = "shl", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = u64::shl, ty1 = u64, ty2 = u64)]
pub struct ShlInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Applies the shift right bitwise operation.
#[inst(name = "shr", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = u64::shr, ty1 = u64, ty2 = u64)]
pub struct ShrInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

/// Applies the arithmetic shift right bitwise operation.
#[inst(name = "shra", operands = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value])]
#[bin_op(op = i64::shr, ty1 = i64, ty2 = u64)]
pub struct ShraInst {
    pub(crate) dest: Operand,
    pub(crate) src1: Operand,
    pub(crate) src2: Operand,
}

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
    fn test_addu_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = AdduInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_addu_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = AdduInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
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
    fn test_addu_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = AdduInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "addu x 2 3");
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
    fn test_subu_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = SubuInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_subu_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = SubuInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
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
    fn test_subu_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = SubuInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "subu x 2 3");
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
    fn test_mulu_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = MuluInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_mulu_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = MuluInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
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
    fn test_mulu_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = MuluInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "mulu x 2 3");
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
    fn test_divu_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = DivuInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_divu_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 1u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = DivuInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
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
    fn test_divu_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = DivuInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "divu x 2 3");
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
    fn test_modu_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = ModuInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_modu_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                if v2 == 0 {
                    continue;
                }

                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = ModuInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
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
    fn test_modu_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = ModuInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "modu x 2 3");
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
    fn test_powu_inst_validation() {
        let d = Operand::Constant(0);
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = PowuInst::new(d, s1, s2);
        inst.validate().unwrap();
    }

    #[test]
    fn test_powu_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u32..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(to_u64!(v2));
                let mut inst = PowuInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
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
    fn test_powu_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = PowuInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "powu x 2 3");
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
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    eq(v1, v2),
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
    fn test_equ_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = EquInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    equ(v1, v2),
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
    fn test_equ_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = EquInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "equ x 2 3");
    }

    #[test]
    fn test_eqf_inst() {
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
                let mut inst = EqfInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    eqf(*v1, *v2),
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
    fn test_eqf_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(0.02_f64));
        let inst = EqfInst::new(d, s1, s2);
        let display = format!("{}", inst);
        let expected = format!("eqf x {} {}", to_u64!(100.14_f64), to_u64!(0.02_f64));
        assert_eq!(display, expected);
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
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    neq(v1, v2),
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
    fn test_nequ_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = NequInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    nequ(v1, v2),
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
    fn test_nequ_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = NequInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "nequ x 2 3");
    }

    #[test]
    fn test_neqf_inst() {
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
                let mut inst = NeqfInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    neqf(*v1, *v2),
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
    fn test_neqf_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(0.02_f64));
        let inst = NeqfInst::new(d, s1, s2);
        let display = format!("{}", inst);
        let expected = format!("neqf x {} {}", to_u64!(100.14_f64), to_u64!(0.02_f64));
        assert_eq!(display, expected);
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
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    lt(v1, v2),
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
    fn test_ltu_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = LtuInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    ltu(v1, v2),
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
    fn test_ltu_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = LtuInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "ltu x 2 3");
    }

    #[test]
    fn test_ltf_inst() {
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
                let mut inst = LtfInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    ltf(*v1, *v2),
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
    fn test_ltf_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(0.02_f64));
        let inst = LtfInst::new(d, s1, s2);
        let display = format!("{}", inst);
        let expected = format!("ltf x {} {}", to_u64!(100.14_f64), to_u64!(0.02_f64));
        assert_eq!(display, expected);
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
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    lte(v1, v2),
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
    fn test_lteu_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = LteuInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    lteu(v1, v2),
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
    fn test_lteu_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = LteuInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "lteu x 2 3");
    }

    #[test]
    fn test_ltef_inst() {
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
                let mut inst = LtefInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    ltef(*v1, *v2),
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
    fn test_ltef_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(0.02_f64));
        let inst = LtefInst::new(d, s1, s2);
        let display = format!("{}", inst);
        let expected = format!("ltef x {} {}", to_u64!(100.14_f64), to_u64!(0.02_f64));
        assert_eq!(display, expected);
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
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    gt(v1, v2),
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
    fn test_gtu_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = GtuInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    gtu(v1, v2),
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
    fn test_gtu_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = GtuInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "gtu x 2 3");
    }

    #[test]
    fn test_gtf_inst() {
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
                let mut inst = GtfInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    gtf(*v1, *v2),
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
    fn test_gtf_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(0.02_f64));
        let inst = GtfInst::new(d, s1, s2);
        let display = format!("{}", inst);
        let expected = format!("gtf x {} {}", to_u64!(100.14_f64), to_u64!(0.02_f64));
        assert_eq!(display, expected);
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
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    gte(v1, v2),
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

    #[test]
    fn test_gteu_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = GteuInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    gteu(v1, v2),
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
    fn test_gteu_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = GteuInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "gteu x 2 3");
    }

    #[test]
    fn test_gtef_inst() {
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
                let mut inst = GtefInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); u8),
                    gtef(*v1, *v2),
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
    fn test_gtef_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(to_u64!(100.14_f64));
        let s2 = Operand::Constant(to_u64!(0.02_f64));
        let inst = GtefInst::new(d, s1, s2);
        let display = format!("{}", inst);
        let expected = format!("gtef x {} {}", to_u64!(100.14_f64), to_u64!(0.02_f64));
        assert_eq!(display, expected);
    }

    #[test]
    fn test_and_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = AndInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    u64::bitand(v1, v2),
                    "{} != {} and {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_and_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = AndInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "and x 2 3");
    }

    #[test]
    fn test_or_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = OrInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    u64::bitor(v1, v2),
                    "{} != {} or {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_or_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = OrInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "or x 2 3");
    }

    #[test]
    fn test_xor_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = XorInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    u64::bitxor(v1, v2),
                    "{} != {} xor {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_xor_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = XorInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "xor x 2 3");
    }

    #[test]
    fn test_shl_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = ShlInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    u64::shl(v1, v2),
                    "{} != {} shl {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_shl_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = ShlInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "shl x 2 3");
    }

    #[test]
    fn test_shr_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0u64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(v1);
                let s2 = Operand::Constant(v2);
                let mut inst = ShrInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    inst.dest.value(&mut context).unwrap(),
                    u64::shr(v1, v2),
                    "{} != {} shr {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_shr_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = ShrInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "shr x 2 3");
    }

    #[test]
    fn test_shra_inst() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);
        for v1 in 0i64..32 {
            for v2 in 0u64..32 {
                i += 1;

                let d = Operand::Variable(String::from("x"));
                let s1 = Operand::Constant(to_u64!(v1));
                let s2 = Operand::Constant(v2);
                let mut inst = ShraInst::new(d, s1, s2);

                inst.execute(&mut context).unwrap();
                inst.inc_inst_ptr(&mut context).unwrap();
                assert_eq!(
                    from_u64!(inst.dest.value(&mut context).unwrap(); i64),
                    i64::shr(v1, v2),
                    "{} != {} shra {}",
                    inst.dest.value(&mut context).unwrap(),
                    v1,
                    v2
                );
                assert_eq!(context.inst_ptr, i);
            }
        }
    }

    #[test]
    fn test_shra_display() {
        let d = Operand::Variable(String::from("x"));
        let s1 = Operand::Constant(2);
        let s2 = Operand::Constant(3);
        let inst = ShraInst::new(d, s1, s2);
        let display = format!("{}", inst);
        assert_eq!(display, "shra x 2 3");
    }
}
