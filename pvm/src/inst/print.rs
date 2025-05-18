use crate::env::Environment;
use crate::inst::*;
use crate::operand::*;
use crate::parse::{Line, Literal, Parse, Token};
use std::fmt::{Debug, Display, Formatter};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;
use crate::from_u64;

#[inst(name = "printi", operands = [OperandFormat::Value])]
#[print(i64)]
pub struct PrintiInst {
    pub(crate) src: Operand,
}

#[inst(name = "printu", operands = [OperandFormat::Value])]
#[print(u64)]
pub struct PrintuInst {
    pub(crate) src: Operand,
}
#[inst(name = "printf", operands = [OperandFormat::Value])]
#[print(f64)]
pub struct PrintfInst {
    pub(crate) src: Operand,
}

#[inst(name = "printh", operands = [OperandFormat::Value])]
pub struct PrinthInst {
    pub(crate) src: Operand,
}

impl Instruction for PrinthInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let val = self.src.value(env)?;
        write!(env.stdout.borrow_mut(), "{:X}", val).unwrap();
        Ok(())
    }
}

impl Display for PrinthInst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", Self::NAME, self.src)
    }
}

#[inst(name = "printb", operands = [OperandFormat::Value])]
pub struct PrintbInst {
    pub(crate) src: Operand,
}

impl Instruction for PrintbInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let val = self.src.value(env)?;
        write!(env.stdout.borrow_mut(), "{:b}", val).unwrap();
        Ok(())
    }
}

impl Display for PrintbInst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", Self::NAME, self.src)
    }
}

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

impl Display for PrintcInst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", Self::NAME, self.src)
    }
}

#[inst(name = "prints", operands = [OperandFormat::Value])]
pub struct PrintsInst {
    pub(crate) src: Operand,
}

impl Instruction for PrintsInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let start = from_u64!(self.src.value(env)?; usize);
        let len: Option<usize> = env.memory.len(start).ok();
        let mut c = env.memory.load_byte(start)?;
        let mut i = 0;
        while c != 0 {
            let ch = char::try_from(c).unwrap_or('?');
            write!(env.stdout.borrow_mut(), "{}", ch).unwrap();
            i += 1;
            if let Some(l) = len {
                if i >= l {
                    break;
                }
            }

            c = env.memory.load_byte(start + i)?;
        }

        Ok(())
    }
}

impl Display for PrintsInst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", Self::NAME, self.src)
    }
}

#[inst(name = "println", operands = [])]
pub struct PrintlnInst {}

impl Instruction for PrintlnInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        writeln!(env.stdout.borrow_mut()).unwrap();
        Ok(())
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
    use crate::inst::*;
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
    fn test_printu_inst_validation() {
        let o = Operand::Label("test".to_string());
        let inst = PrintuInst::new(o);
        inst.validate().unwrap();
    }

    #[test]
    fn test_printu_inst() {
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, buffer.clone());

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
    fn test_printh_inst() {
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, buffer.clone());

        let mut inst = PrinthInst::new(Operand::Constant(10));
        inst.execute(&mut context).unwrap();

        let output: String = buffer.borrow().iter().map(|b| *b as char).collect();
        assert_eq!("A", output);
    }

    #[test]
    fn test_printb_inst() {
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, buffer.clone());

        let mut inst = PrintbInst::new(Operand::Constant(10));
        inst.execute(&mut context).unwrap();

        let output: String = buffer.borrow().iter().map(|b| *b as char).collect();
        assert_eq!("1010", output);
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
    fn test_prints_inst() {
        let buffer: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut context = Environment::new(32, buffer.clone());

        let value = "Hello World!".to_string();

        // create the string
        let addr = Operand::Variable("addr".to_string());
        let mut inst = AllocInst::new(addr.clone(), Operand::Constant(to_u64!(value.len())));
        inst.execute(&mut context).unwrap();

        for (i, c) in value.bytes().enumerate() {
            let x = Operand::Variable("x".to_string());
            let mut inst = AddInst::new(x.clone(), addr.clone(), Operand::Constant(to_u64!(i)));
            inst.execute(&mut context).unwrap();

            let mut inst = StoreByteInst::new(x.clone(), Operand::Constant(to_u64!(c)));
            inst.execute(&mut context).unwrap();
        }

        let mut inst = PrintsInst::new(addr.clone());
        inst.execute(&mut context).unwrap();

        let output: String = buffer.borrow().iter().map(|b| *b as char).collect();
        assert_eq!(value, output);
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
