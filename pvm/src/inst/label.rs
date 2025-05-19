use pvm_proc_macros::Inst;
use crate::env::Environment;
use crate::inst::*;
use crate::parse::{Line, Literal, Parse, Token};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;
use crate::inst::error::ValidateError;

/// Creates a label.
#[inst(name = "label", operands = [OperandFormat::Label])]
pub struct LabelInst {
    pub(crate) lab: Operand,
}

impl Instruction for LabelInst {
    fn execute(&mut self, _env: &mut Environment) -> Result<(), Error> {
        Ok(())
    }
    
    fn initialize(&self, env: &mut Environment, i: usize) -> Result<(), Error> {
        let label = self.lab.label()?;
        if env.labels.contains_key(&label) {
            return Err(ValidateError::label_already_defined(&label));
        }

        env.labels.insert(label, i+1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_label_validation() {
        let d = Operand::Constant(0);
        let inst = LabelInst::new(d);
        inst.validate().unwrap();
    }

    #[test]
    fn test_label_initialization() {
        let mut context = Environment::default();
        let inst = LabelInst::new(Operand::Label("test".to_string()));
        inst.initialize(&mut context, 0).unwrap();
        assert_eq!(context.labels.get("test"), Some(&1));
    }

    #[test]
    fn test_label_display() {
        let lab = "test".to_string();
        let d = Operand::Label(lab.clone());
        let inst = LabelInst::new(d);
        let display = format!("{}", inst);
        assert_eq!(display, "label test");
    }
}