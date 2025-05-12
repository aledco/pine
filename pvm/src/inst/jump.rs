use pvm_proc_macros::Inst;
use crate::env::Environment;
use crate::inst::Instruction;
use crate::operand::*;
use crate::parse::{Line, Literal, Parse, Token};
use std::fmt::{Debug, Display, Formatter};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

#[inst(name = "jump", operands = [OperandFormat::Label])]
pub struct JumpInst {
    pub src: Operand,
}

impl Instruction for JumpInst {
    fn execute(&mut self, _env: &mut Environment) -> Result<(), String> {
        Ok(())
    }

    fn inc_inst_ptr(&self, env: &mut Environment) -> Result<(), String> {
        let label = self.src.label()?;
        let addr = env.labels.get(&label);
        match addr {
            Some(addr) => {
                env.inst_ptr = *addr;
                Ok(())
            },
            None => Err(format!("Label {} does not exist", label))?,
        }
    }
    
    fn validate(&self) -> Result<(), String> {
        if !matches!(self.src, Operand::Label(_)) {
            Err("src must be a label".to_string())
        } else {
            Ok(())
        }
    }
}

impl Display for JumpInst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", Self::NAME, self.src)
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
}
