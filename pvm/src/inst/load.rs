use pvm_proc_macros::Inst;
use crate::env::Environment;
use crate::inst::*;
use crate::operand::*;
use crate::cast::*;
use crate::parse::{Line, Literal, Parse, Token};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

/// Loads a value from memory.
#[inst(name = "load", operands = [OperandFormat::Variable, OperandFormat::Variable])]
pub struct LoadInst {
    pub(crate) dest: Operand,
    pub(crate) src: Operand,
}

impl Instruction for LoadInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let addr = from_u64!(self.src.value(env)?; usize);
        let value = env.memory.load(addr)?;
        self.dest.set_value(value, env)?;
        Ok(())
    }
}

/// Loads a byte from memory.
#[inst(name = "loadb", operands = [OperandFormat::Variable, OperandFormat::Variable])]
pub struct LoadByteInst {
    pub(crate) dest: Operand,
    pub(crate) src: Operand,
}

impl Instruction for LoadByteInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let addr = from_u64!(self.src.value(env)?; usize);
        let value = to_u64!(env.memory.load_byte(addr)?);
        self.dest.set_value(value, env)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ExecuteConfig;
    use crate::inst::store::StoreInst;
    use super::*;

    #[test]
    #[should_panic]
    fn test_load_validation() {
        let d = Operand::Constant(0);
        let s = Operand::Constant(2);
        let inst = LoadInst::new(d, s);
        inst.validate().unwrap();
    }

    #[test]
    fn test_load() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);

        let vals: Vec<i64> = vec![-100, -10, 0, 4, 5, 10, 20];
        for v in &vals {
            i += 1;

            // create alloc
            let d = Operand::Variable(String::from("x"));
            let s = Operand::Constant(8);
            let mut inst = AllocInst::new(d, s);
            inst.execute(&mut context).unwrap();
            inst.inc_inst_ptr(&mut context).unwrap();
            assert_eq!(context.inst_ptr, i);

            // create store inst
            i += 1;
            let d = Operand::Variable(String::from("x"));
            let s = Operand::Constant(to_u64!(*v));
            let mut inst = StoreInst::new(d, s);
            inst.execute(&mut context).unwrap();
            inst.inc_inst_ptr(&mut context).unwrap();
            assert_eq!(context.inst_ptr, i);

            // test load inst
            i += 1;
            let d = Operand::Variable(String::from("y"));
            let s = Operand::Variable(String::from("x"));
            let mut inst = LoadInst::new(d, s);
            inst.execute(&mut context).unwrap();
            inst.inc_inst_ptr(&mut context).unwrap();
            let value = inst.dest.value(&mut context).unwrap();
            assert_eq!(from_u64!(value; i64), *v);
            assert_eq!(context.inst_ptr, i);
        }
    }

    #[test]
    fn test_load_display() {
        let d = Operand::Variable(String::from("x"));
        let s = Operand::Variable(String::from("y"));
        let inst = LoadInst::new(d, s);
        let display = format!("{}", inst);
        assert_eq!(display, "load x y");
    }

    #[test]
    #[should_panic]
    fn test_loadb_validation() {
        let d = Operand::Constant(0);
        let s = Operand::Constant(2);
        let inst = LoadByteInst::new(d, s);
        inst.validate().unwrap();
    }

    #[test]
    fn test_loadb() {
        let mut i = 0;
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);

        let vals: Vec<i8> = vec![-100, -10, 0, 4, 5, 10, 20];
        for v in &vals {
            i += 1;

            // create alloc
            let d = Operand::Variable(String::from("x"));
            let s = Operand::Constant(1);
            let mut inst = AllocInst::new(d, s);
            inst.execute(&mut context).unwrap();
            inst.inc_inst_ptr(&mut context).unwrap();
            assert_eq!(context.inst_ptr, i);

            // create store inst
            i += 1;
            let d = Operand::Variable(String::from("x"));
            let s = Operand::Constant(to_u64!(*v));
            let mut inst = StoreByteInst::new(d, s);
            inst.execute(&mut context).unwrap();
            inst.inc_inst_ptr(&mut context).unwrap();
            assert_eq!(context.inst_ptr, i);

            // test load inst
            i += 1;
            let d = Operand::Variable(String::from("y"));
            let s = Operand::Variable(String::from("x"));
            let mut inst = LoadByteInst::new(d, s);
            inst.execute(&mut context).unwrap();
            inst.inc_inst_ptr(&mut context).unwrap();
            let value = inst.dest.value(&mut context).unwrap();
            assert_eq!(from_u64!(value; i8), *v);
            assert_eq!(context.inst_ptr, i);
        }
    }

    #[test]
    fn test_loadb_display() {
        let d = Operand::Variable(String::from("x"));
        let s = Operand::Variable(String::from("y"));
        let inst = LoadByteInst::new(d, s);
        let display = format!("{}", inst);
        assert_eq!(display, "loadb x y");
    }
}