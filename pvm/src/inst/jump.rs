use pvm_proc_macros::Inst;
use crate::env::Environment;
use crate::inst::*;
use crate::operand::*;
use crate::parse::{Line, Literal, Parse, Token};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

#[inst(name = "jump", operands = [OperandFormat::Label])]
pub struct JumpInst {
    pub(crate) lab: Operand,
}

impl Instruction for JumpInst {
    fn execute(&mut self, _env: &mut Environment) -> Result<(), Error> {
        Ok(())
    }

    fn inc_inst_ptr(&self, env: &mut Environment) -> Result<(), Error> {
        let label = self.lab.label()?;
        let addr = env.labels.get(&label);
        match addr {
            Some(addr) => {
                env.inst_ptr = *addr;
                Ok(())
            },
            None => Err(ExecuteError::label_does_not_exist(&label)),
        }
    }
}

#[inst(name = "jumpz", operands = [OperandFormat::Value, OperandFormat::Label])]
pub struct JumpZeroInst {
    pub(crate) src: Operand,
    pub(crate) lab: Operand,
}

impl Instruction for JumpZeroInst {
    fn execute(&mut self, _env: &mut Environment) -> Result<(), Error> {
        Ok(())
    }

    fn inc_inst_ptr(&self, env: &mut Environment) -> Result<(), Error> {
        let value = self.src.value(env)?;
        if value == 0 {
            let label = self.lab.label()?;
            let addr = env.labels.get(&label);

            match addr {
                Some(addr) => {
                    env.inst_ptr = *addr;
                    Ok(())
                },
                None => Err(ExecuteError::label_does_not_exist(&label)),
            }
        } else {
            env.inst_ptr += 1;
            Ok(())
        }
    }
}

#[inst(name = "jumpnz", operands = [OperandFormat::Value, OperandFormat::Label])]
pub struct JumpNotZeroInst {
    pub(crate) src: Operand,
    pub(crate) lab: Operand,
}

impl Instruction for JumpNotZeroInst {
    fn execute(&mut self, _env: &mut Environment) -> Result<(), Error> {
        Ok(())
    }

    fn inc_inst_ptr(&self, env: &mut Environment) -> Result<(), Error> {
        let value = self.src.value(env)?;
        if value != 0 {
            let label = self.lab.label()?;
            let addr = env.labels.get(&label);

            match addr {
                Some(addr) => {
                    env.inst_ptr = *addr;
                    Ok(())
                },
                None => Err(ExecuteError::label_does_not_exist(&label)),
            }
        } else {
            env.inst_ptr += 1;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ExecuteConfig;
    use super::*;

    #[test]
    #[should_panic]
    fn test_jump_validation() {
        let d = Operand::Constant(0);
        let inst = JumpInst::new(d);
        inst.validate().unwrap();
    }

    #[test]
    fn test_jump_inst() {
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);

        let lab = "test".to_string();
        context.labels.insert(lab.clone(), 100);
        
        let d = Operand::Label(lab.clone());
        let mut inst = JumpInst::new(d);
        
        inst.execute(&mut context).unwrap();
        inst.inc_inst_ptr(&mut context).unwrap();

        assert_eq!(context.inst_ptr, 100);
    }

    #[test]
    fn test_jump_display() {
        let lab = "test".to_string();
        let d = Operand::Label(lab.clone());
        let inst = JumpInst::new(d);
        let display = format!("{}", inst);
        assert_eq!(display, "jump test");
    }

    #[test]
    #[should_panic]
    fn test_jumpz_validation() {
        let s = Operand::Constant(0);
        let l = Operand::Constant(0);
        let inst = JumpZeroInst::new(s, l);
        inst.validate().unwrap();
    }

    #[test]
    fn test_jumpz_inst() {
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);

        let lab = "test".to_string();
        context.labels.insert(lab.clone(), 100);

        let s1 = Operand::Constant(0);
        let l = Operand::Label(lab.clone());
        let mut inst = JumpZeroInst::new(s1, l);
        inst.execute(&mut context).unwrap();
        inst.inc_inst_ptr(&mut context).unwrap();
        assert_eq!(context.inst_ptr, 100);

        let s1 = Operand::Constant(1);
        let l = Operand::Label(lab.clone());
        let mut inst = JumpZeroInst::new(s1, l);
        inst.execute(&mut context).unwrap();
        inst.inc_inst_ptr(&mut context).unwrap();
        assert_eq!(context.inst_ptr, 101);
    }

    #[test]
    fn test_jumpz_display() {
        let s = Operand::Constant(0);
        let lab = "test".to_string();
        let l = Operand::Label(lab.clone());
        let inst = JumpZeroInst::new(s, l);
        let display = format!("{}", inst);
        assert_eq!(display, "jumpz 0 test");
    }

    #[test]
    #[should_panic]
    fn test_jumpnz_validation() {
        let s = Operand::Constant(0);
        let l = Operand::Constant(0);
        let inst = JumpNotZeroInst::new(s, l);
        inst.validate().unwrap();
    }

    #[test]
    fn test_jumpnz_inst() {
        let config = ExecuteConfig::default();
        let mut context = Environment::new(config.memory_size, config.stdout);

        let lab = "test".to_string();
        context.labels.insert(lab.clone(), 100);

        let s1 = Operand::Constant(1);
        let l = Operand::Label(lab.clone());
        let mut inst = JumpNotZeroInst::new(s1, l);
        inst.execute(&mut context).unwrap();
        inst.inc_inst_ptr(&mut context).unwrap();
        assert_eq!(context.inst_ptr, 100);

        let s1 = Operand::Constant(0);
        let l = Operand::Label(lab.clone());
        let mut inst = JumpNotZeroInst::new(s1, l);
        inst.execute(&mut context).unwrap();
        inst.inc_inst_ptr(&mut context).unwrap();
        assert_eq!(context.inst_ptr, 101);
    }

    #[test]
    fn test_jumpnz_display() {
        let s = Operand::Constant(0);
        let lab = "test".to_string();
        let l = Operand::Label(lab.clone());
        let inst = JumpNotZeroInst::new(s, l);
        let display = format!("{}", inst);
        assert_eq!(display, "jumpnz 0 test");
    }
}
