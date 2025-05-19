use crate::env::Environment;
use crate::inst::*;
use crate::parse::{Line, Literal, Parse, Token};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;
use crate::from_u64;

/// Prints a signed integer.
#[inst(name = "printi", operands = [OperandFormat::Value])]
#[print(i64)]
pub struct PrintiInst {
    pub(crate) src: Operand,
}

/// Prints an unsigned integer.
#[inst(name = "printu", operands = [OperandFormat::Value])]
#[print(u64)]
pub struct PrintuInst {
    pub(crate) src: Operand,
}

/// Prints a floating point number.
#[inst(name = "printf", operands = [OperandFormat::Value])]
#[print(f64)]
pub struct PrintfInst {
    pub(crate) src: Operand,
}

/// Prints a hexadecimal integer.
#[inst(name = "printh", operands = [OperandFormat::Value])]
pub struct PrinthInst {
    pub(crate) src: Operand,
}

impl Instruction for PrinthInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let val = self.src.value(env)?;
        write!(env.stdout.borrow_mut(), "0x{:X}", val).unwrap();
        Ok(())
    }
}

/// Prints a binary integer.
#[inst(name = "printb", operands = [OperandFormat::Value])]
pub struct PrintbInst {
    pub(crate) src: Operand,
}

impl Instruction for PrintbInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let val = self.src.value(env)?;
        write!(env.stdout.borrow_mut(), "0b{:b}", val).unwrap();
        Ok(())
    }
}

/// Prints an ascii character.
#[inst(name = "printc", operands = [OperandFormat::Value])]
pub struct PrintcInst {
    pub(crate) src: Operand,
}

impl Instruction for PrintcInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let value = from_u64!(self.src.value(env)?; u8);
        let c = char::try_from(value).unwrap_or('?');
        write!(env.stdout.borrow_mut(), "{}", c).unwrap();
        Ok(())
    }
}

/// Prints a newline character.
#[inst(name = "println", operands = [])]
pub struct PrintlnInst {}

impl Instruction for PrintlnInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        writeln!(env.stdout.borrow_mut()).unwrap();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::to_u64;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    #[should_panic]
    fn test_printi_inst_validation() {
        let o = Operand::Label("test".to_string());
        let inst = PrintiInst::new(o);
        inst.validate().unwrap();
    }

    #[test]
    fn test_printi_inst() {
        let stdin = Rc::new(RefCell::new(std::io::stdin()));
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, stdin, buffer.clone());

        let mut inst = PrintiInst::new(Operand::Constant(10));
        inst.execute(&mut context).unwrap();

        let output: String = buffer.borrow().iter().map(|b| *b as char).collect();
        assert_eq!("10", output);
    }

    #[test]
    fn test_printi_inst_display() {
        let inst = PrintiInst::new(Operand::Constant(10));
        let d = format!("{}", inst);
        assert_eq!("printi 10", d);
    }

    #[test]
    #[should_panic]
    fn test_printu_inst_validation() {
        let o = Operand::Label("test".to_string());
        let inst = PrintuInst::new(o);
        inst.validate().unwrap();
    }

    #[test]
    fn test_printu_inst() {
        let stdin = Rc::new(RefCell::new(std::io::stdin()));
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, stdin, buffer.clone());

        let mut inst = PrintuInst::new(Operand::Constant(10));
        inst.execute(&mut context).unwrap();

        let output: String = buffer.borrow().iter().map(|b| *b as char).collect();
        assert_eq!("10", output);
    }

    #[test]
    fn test_printu_inst_display() {
        let inst = PrintuInst::new(Operand::Constant(10));
        let d = format!("{}", inst);
        assert_eq!("printu 10", d);
    }

    #[test]
    #[should_panic]
    fn test_printf_inst_validation() {
        let o = Operand::Label("test".to_string());
        let inst = PrintfInst::new(o);
        inst.validate().unwrap();
    }

    #[test]
    fn test_printf_inst() {
        let stdin = Rc::new(RefCell::new(std::io::stdin()));
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, stdin, buffer.clone());

        let mut inst = PrintfInst::new(Operand::Constant(to_u64!(100.14_f64)));
        inst.execute(&mut context).unwrap();

        let output: String = buffer.borrow().iter().map(|b| *b as char).collect();
        assert_eq!("100.14", output);
    }

    #[test]
    fn test_printf_inst_display() {
        let inst = PrintfInst::new(Operand::Constant(to_u64!(100.14_f64)));
        let d = format!("{}", inst);
        let e = format!("printf {}", to_u64!(100.14_f64));
        assert_eq!(e, d);
    }

    #[test]
    fn test_printh_inst() {
        let stdin = Rc::new(RefCell::new(std::io::stdin()));
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, stdin, buffer.clone());

        let mut inst = PrinthInst::new(Operand::Constant(10));
        inst.execute(&mut context).unwrap();

        let output: String = buffer.borrow().iter().map(|b| *b as char).collect();
        assert_eq!("0xA", output);
    }

    #[test]
    fn test_printb_inst() {
        let stdin = Rc::new(RefCell::new(std::io::stdin()));
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, stdin, buffer.clone());

        let mut inst = PrintbInst::new(Operand::Constant(10));
        inst.execute(&mut context).unwrap();

        let output: String = buffer.borrow().iter().map(|b| *b as char).collect();
        assert_eq!("0b1010", output);
    }

    #[test]
    #[should_panic]
    fn test_printc_inst_validation() {
        let o = Operand::Label("test".to_string());
        let inst = PrintcInst::new(o);
        inst.validate().unwrap();
    }

    #[test]
    fn test_printc_inst() {
        let stdin = Rc::new(RefCell::new(std::io::stdin()));
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, stdin, buffer.clone());

        let mut inst = PrintcInst::new(Operand::Constant(97));
        inst.execute(&mut context).unwrap();

        let output: String = buffer.borrow().iter().map(|b| *b as char).collect();
        assert_eq!("a", output);
    }

    #[test]
    fn test_printc_inst_display() {
        let inst = PrintcInst::new(Operand::Constant(97));
        let d = format!("{}", inst);
        assert_eq!("printc 97", d);
    }

    #[test]
    fn test_println_inst() {
        let stdin = Rc::new(RefCell::new(std::io::stdin()));
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, stdin, buffer.clone());

        let mut inst = PrintlnInst::new();
        inst.execute(&mut context).unwrap();

        let output: String = buffer.borrow().iter().map(|b| *b as char).collect();
        assert_eq!("\n", output);
    }

    #[test]
    fn test_println_inst_display() {
        let inst = PrintlnInst::new();
        let d = format!("{}", inst);
        assert_eq!("println", d);
    }
}
