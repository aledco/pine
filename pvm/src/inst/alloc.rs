use pvm_proc_macros::Inst;
use crate::env::Environment;
use crate::inst::*;
use crate::operand::*;
use crate::cast::*;
use crate::parse::{Line, Literal, Parse, Token};
use std::fmt::{Debug, Display, Formatter};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

#[inst(name = "alloc", operands = [OperandFormat::Variable, OperandFormat::Value])]
pub struct AllocInst {
    pub(crate) dest: Operand,
    pub(crate) src: Operand,
}

impl Instruction for AllocInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let size = from_u64!(self.src.value(env)?; usize);
        let addr = to_u64!(env.memory.allocate(size)?);
        self.dest.set_value(addr, env)?;
        Ok(())
    }
}

impl Display for AllocInst { // TODO can auto derive this in inst too
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", Self::NAME, self.dest, self.src)
    }
}

#[inst(name = "dealloc", operands = [OperandFormat::Variable])]
pub struct DeallocInst {
    pub(crate) src: Operand,
}

impl Instruction for DeallocInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let addr = from_u64!(self.src.value(env)?; usize);
        env.memory.deallocate(addr)?;
        Ok(())
    }
}

impl Display for DeallocInst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", Self::NAME, self.src)
    }
}

#[inst(name = "len", operands = [OperandFormat::Variable, OperandFormat::Variable])]
pub struct LenInst {
    pub(crate) dest: Operand,
    pub(crate) src: Operand,
}

impl Instruction for LenInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let addr = from_u64!(self.src.value(env)?; usize);
        let len = to_u64!(env.memory.len(addr)?);
        self.dest.set_value(len, env)?;
        Ok(())
    }
}

impl Display for LenInst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", Self::NAME, self.dest, self.src)
    }
}

#[cfg(test)]
mod tests {
    use crate::ExecuteConfig;
    use super::*;

    #[test]
    #[should_panic]
    fn test_alloc_validation() {
        let d = Operand::Constant(0);
        let s = Operand::Constant(2);
        let inst = AllocInst::new(d, s);
        inst.validate().unwrap();
    }

    #[test]
    fn test_alloc() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);

        let vals: Vec<usize> = vec![1, 2, 5, 10, 20];
        for v in &vals {
            i += 1;

            let d = Operand::Variable(String::from("x"));
            let s = Operand::Constant(to_u64!(*v));
            let mut inst = AllocInst::new(d, s);

            inst.execute(&mut context).unwrap();
            inst.inc_inst_ptr(&mut context).unwrap();
            let _ = from_u64!(inst.dest.value(&mut context).unwrap(); usize); // no way to verify address
            assert_eq!(context.inst_ptr, i);
        }
    }

    #[test]
    fn test_alloc_display() {
        let d = Operand::Variable(String::from("x"));
        let s = Operand::Constant(2);
        let inst = AllocInst::new(d, s);
        let display = format!("{}", inst);
        assert_eq!(display, "alloc x 2");
    }

    #[test]
    #[should_panic]
    fn test_dealloc_validation() {
        let d = Operand::Constant(0);
        let inst = DeallocInst::new(d);
        inst.validate().unwrap();
    }

    #[test]
    fn test_dealloc() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);

        let vals: Vec<usize> = vec![1, 2, 5, 10, 20];
        for v in &vals {
            i += 1;

            // create allocation
            let d = Operand::Variable(String::from("x"));
            let s = Operand::Constant(to_u64!(*v));
            let mut inst = AllocInst::new(d, s);
            inst.execute(&mut context).unwrap();
            inst.inc_inst_ptr(&mut context).unwrap();
            let addr = from_u64!(inst.dest.value(&mut context).unwrap(); usize);
            assert_eq!(context.inst_ptr, i);
            i += 1;

            // test deallocation
            let d = Operand::Constant(to_u64!(addr));
            let mut inst = DeallocInst::new(d);
            inst.execute(&mut context).unwrap();
            inst.inc_inst_ptr(&mut context).unwrap(); // no way to verify deallocation
            assert_eq!(context.inst_ptr, i);
        }
    }

    #[test]
    fn test_dealloc_display() {
        let d = Operand::Variable(String::from("x"));
        let inst = DeallocInst::new(d);
        let display = format!("{}", inst);
        assert_eq!(display, "dealloc x");
    }

    #[test]
    #[should_panic]
    fn test_len_validation() {
        let d = Operand::Constant(0);
        let s = Operand::Constant(2);
        let inst = LenInst::new(d, s);
        inst.validate().unwrap();
    }

    #[test]
    fn test_len() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);

        let vals: Vec<usize> = vec![1, 2, 5, 10, 20];
        for v in vals {
            i += 1;

            // create allocation
            let d = Operand::Variable(String::from("x"));
            let s = Operand::Constant(to_u64!(v));
            let mut inst = AllocInst::new(d, s);
            inst.execute(&mut context).unwrap();
            inst.inc_inst_ptr(&mut context).unwrap();
            let _ = from_u64!(inst.dest.value(&mut context).unwrap(); usize);
            assert_eq!(context.inst_ptr, i);

            // test len inst
            i += 1;
            let d = Operand::Variable(String::from("y"));
            let s = Operand::Variable(String::from("x"));
            let mut inst = LenInst::new(d, s);
            inst.execute(&mut context).unwrap();
            inst.inc_inst_ptr(&mut context).unwrap();
            let len = from_u64!(inst.dest.value(&mut context).unwrap(); usize);
            assert_eq!(len, v);
            assert_eq!(context.inst_ptr, i);
        }
    }

    #[test]
    fn test_len_display() {
        let d = Operand::Variable(String::from("x"));
        let s = Operand::Variable(String::from("y"));
        let inst = LenInst::new(d, s);
        let display = format!("{}", inst);
        assert_eq!(display, "len x y");
    }
}
