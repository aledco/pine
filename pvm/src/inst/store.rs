use pvm_proc_macros::Inst;
use crate::env::Environment;
use crate::inst::*;
use crate::operand::*;
use crate::cast::*;
use crate::parse::{Line, Literal, Parse, Token};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

#[inst(name = "store", operands = [OperandFormat::Variable, OperandFormat::Value])]
pub struct StoreInst {
    pub(crate) dest: Operand,
    pub(crate) src: Operand,
}

impl Instruction for StoreInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let value = self.src.value(env)?;
        let addr = from_u64!(self.dest.value(env)?; usize);
        env.memory.store(addr, value)?;
        Ok(())
    }
}

#[inst(name = "storeb", operands = [OperandFormat::Variable, OperandFormat::Value])]
pub struct StoreByteInst {
    pub(crate) dest: Operand,
    pub(crate) src: Operand,
}

impl Instruction for StoreByteInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let value = from_u64!(self.src.value(env)?; u8);
        let addr = from_u64!(self.dest.value(env)?; usize);
        env.memory.store_byte(addr, value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ExecuteConfig;
    use super::*;

    #[test]
    #[should_panic]
    fn test_store_validation() {
        let d = Operand::Constant(0);
        let s = Operand::Constant(2);
        let inst = StoreInst::new(d, s);
        inst.validate().unwrap();
    }

    #[test]
    fn test_store() {
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

            // test with load inst
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
    fn test_store_display() {
        let d = Operand::Variable(String::from("x"));
        let s = Operand::Variable(String::from("y"));
        let inst = StoreInst::new(d, s);
        let display = format!("{}", inst);
        assert_eq!(display, "store x y");
    }

    #[test]
    #[should_panic]
    fn test_storeb_validation() {
        let d = Operand::Constant(0);
        let s = Operand::Constant(2);
        let inst = StoreByteInst::new(d, s);
        inst.validate().unwrap();
    }

    #[test]
    fn test_storeb() {
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

            // test with load inst
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
    fn test_storeb_display() {
        let d = Operand::Variable(String::from("x"));
        let s = Operand::Variable(String::from("y"));
        let inst = StoreByteInst::new(d, s);
        let display = format!("{}", inst);
        assert_eq!(display, "storeb x y");
    }
}