use crate::env::Environment;
use crate::inst::Instruction;
use crate::operand::*;
use crate::parse::{Line, Literal, Parse, Token};
use std::fmt::{Debug, Display, Formatter};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

#[inst(name = "printi", operands = [OperandFormat::Value])]
#[print(i64)]
pub struct PrintiInst {}

#[inst(name = "printf", operands = [OperandFormat::Value])]
#[print(f64)]
pub struct PrintfInst {}

#[inst(name = "printc", operands = [OperandFormat::Value])]
pub struct PrintcInst {
    pub src: Operand,
}

impl Instruction for PrintcInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), String> {
        let value = crate::cast::from_u64!(self.src.value(env)?; u8);
        let res = {
            let c = char::try_from(value).unwrap_or('?');
            write!(env.stdout.borrow_mut(), "{}", c)
        };
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }
    
    fn validate(&self) -> Result<(), String> {
        if matches!(self.src, Operand::Label(_)) {
            Err("src must be a variable or constant".to_string())
        } else {
            Ok(())
        }
    }
}

impl Display for PrintcInst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", Self::NAME, self.src)
    }
}

#[inst(name = "println", operands = [])]
pub struct PrintlnInst {}

impl Instruction for PrintlnInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), String> {
        let res = { writeln!(env.stdout.borrow_mut()) };
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }
}

impl Display for PrintlnInst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", Self::NAME)
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
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, buffer.clone());

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
    fn test_printf_inst_validation() {
        let o = Operand::Label("test".to_string());
        let inst = PrintfInst::new(o);
        inst.validate().unwrap();
    }

    #[test]
    fn test_printf_inst() {
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, buffer.clone());

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
    #[should_panic]
    fn test_printc_inst_validation() {
        let o = Operand::Label("test".to_string());
        let inst = PrintcInst::new(o);
        inst.validate().unwrap();
    }

    #[test]
    fn test_printc_inst() {
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, buffer.clone());

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
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, buffer.clone());

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
