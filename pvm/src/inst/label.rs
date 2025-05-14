use pvm_proc_macros::Inst;
use crate::env::Environment;
use crate::inst::*;
use crate::operand::*;
use crate::parse::{Line, Literal, Parse, Token};
use std::fmt::{Debug, Display, Formatter};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

#[inst(name = "label", operands = [OperandFormat::Label])]
pub struct LabelInst {
    pub lab: Operand,
}

impl Instruction for LabelInst {
    fn execute(&mut self, _env: &mut Environment) -> Result<(), String> {
        Ok(())
    }

    fn defined_label(&self) -> Option<String> {
        match self.lab.label() {
            Ok(label) => Some(label),
            Err(_) => None
        }
    }
}

impl Display for LabelInst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", Self::NAME, self.lab)
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
    fn test_label_display() {
        let lab = "test".to_string();
        let d = Operand::Label(lab.clone());
        let inst = LabelInst::new(d);
        let display = format!("{}", inst);
        assert_eq!(display, "label test");
    }
}