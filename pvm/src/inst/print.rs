use crate::env::Environment;
use crate::operand::*;
use crate::parse::{Parse, Line, Token, Literal};
use crate::inst::Instruction;
use std::fmt::{Debug, Display, Formatter};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

#[inst(name = "printi", operands = [OperandFormat::Value])]
pub struct PrintiInst {
    pub src: Operand,
}

impl Instruction for PrintiInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), String> {
        let value = self.src.value(env)?;
        let res = write!(env.stdout.borrow_mut(), "{}", value);
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }

    fn used_vars(&self) -> Vec<Operand> {
        if let Operand::Variable(_) = self.src {
            return vec![self.src.clone()];
        }

        vec![]
    }

    fn validate(&self) -> Result<(), String> {
        if matches!(self.src, Operand::Label(_)) {
            Err("src must be a variable or constant".to_string())
        } else {
            Ok(())
        }
    }
}

impl Display for PrintiInst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", Self::NAME, self.src)
    }
}

#[inst(name = "printc", operands = [OperandFormat::Value])]
pub struct PrintcInst {
    pub src: Operand,
}

impl Instruction for PrintcInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), String> {
        let value = self.src.value(env)?;
        let res = {
            let c = char::try_from(value as u32).unwrap_or('?');
            write!(env.stdout.borrow_mut(), "{}", c)
        };
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{}", e)),
        }
    }

    fn used_vars(&self) -> Vec<Operand> {
        if let Operand::Variable(_) = self.src {
            return vec![self.src.clone()];
        }

        vec![]
    }

    fn validate(&self) -> Result<(), String> {
        if matches!(self.src, Operand::Label(_)) {
            Err("src must be a variable or constant".to_string())
        } else {
            Ok(())
        }
    }
}

#[inst(name = "println", operands = [])]
pub struct PrintlnInst {}

impl Instruction for PrintlnInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), String> {
        let res = {
            writeln!(env.stdout.borrow_mut())
        };
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

impl Display for PrintcInst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", Self::NAME, self.src)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::*;

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

        let output: String = buffer
            .borrow()
            .iter()
            .map(|b| *b as char)
            .collect();
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

        let output: String = buffer
            .borrow()
            .iter()
            .map(|b| *b as char)
            .collect();
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

        let output: String = buffer
            .borrow()
            .iter()
            .map(|b| *b as char)
            .collect();
        assert_eq!("\n", output);
    }

    #[test]
    fn test_println_inst_display() {
        let inst = PrintlnInst::new();
        let d = format!("{}", inst);
        assert_eq!("println", d);
    }
}