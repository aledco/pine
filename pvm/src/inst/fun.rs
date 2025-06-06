use std::collections::HashMap;
use pvm_proc_macros::Inst;
use crate::env::Environment;
use crate::inst::*;
use crate::parse::{Line, Literal, Parse, Token};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

/// Creates a label for a function.
#[inst(name = "fun", operands = [OperandFormat::Label])]
pub struct FunInst {
    pub(crate) lab: Operand,
}

impl Instruction for FunInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        env.push_variable_stack();
        Ok(())
    }

    fn initialize(&self, env: &mut Environment, i: usize) -> Result<(), Error> {
        let label = self.lab.label()?;
        if env.fun_labels.contains_key(&label) {
            return Err(ValidateError::label_already_defined(&label));
        }

        env.fun_labels.insert(label, i);
        Ok(())
    }
}

/// Pushes an argument to the argument queue.
#[inst(name = "pusha", operands = [OperandFormat::Value])]
pub struct PushaInst {
    pub(crate) src: Operand,
}

impl Instruction for PushaInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let val = self.src.value(env)?;
        env.arg_queue.push_back(val);
        Ok(())
    }
}

/// Pops an argument from the argument queue.
#[inst(name = "popa", operands = [OperandFormat::Variable])]
pub struct PopaInst {
    pub(crate) dest: Operand,
}

impl Instruction for PopaInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let val = match env.arg_queue.pop_front() {
            Some(v) => v,
            None => return Err(ExecuteError::arg_queue_is_empty())
        };

        self.dest.set_value(val, env)?;
        Ok(())
    }
}

/// Pushes a return value to the ret val queue.
#[inst(name = "pushr", operands = [OperandFormat::Value])]
pub struct PushrInst {
    pub(crate) src: Operand,
}

impl Instruction for PushrInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let val = self.src.value(env)?;
        env.ret_queue.push_back(val);
        Ok(())
    }
}

/// Pops a return value from the ret val queue.
#[inst(name = "popr", operands = [OperandFormat::Variable])]
pub struct PoprInst {
    pub(crate) dest: Operand,
}

impl Instruction for PoprInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let val = match env.ret_queue.pop_front() {
            Some(v) => v,
            None => return Err(ExecuteError::ret_queue_is_empty())
        };

        self.dest.set_value(val, env)?;
        Ok(())
    }
}

/// Calls a function.
#[inst(name = "call", operands = [OperandFormat::Label])]
pub struct CallInst {
    pub(crate) lab: Operand,
}

impl Instruction for CallInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        env.ret_queue.clear();
        env.ret_addr_stack.push(env.inst_ptr + 1);
        Ok(())
    }

    fn inc_inst_ptr(&self, env: &mut Environment) -> Result<(), Error> {
        let label = self.lab.label()?;
        let addr = env.fun_labels.get(&label);
        match addr {
            Some(addr) => {
                env.inst_ptr = *addr;
                Ok(())
            },
            None => Err(ExecuteError::label_does_not_exist(&label)),
        }
    }
}

/// Returns from a function to the call point.
#[inst(name = "ret", operands = [])]
pub struct RetInst {
}

impl Instruction for RetInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        env.arg_queue.clear();
        env.pop_variable_stack();
        Ok(())
    }

    fn inc_inst_ptr(&self, env: &mut Environment) -> Result<(), Error> {
        let addr = env.ret_addr_stack.pop();
        match addr {
            Some(addr) => {
                env.inst_ptr = addr;
                Ok(())
            },
            None => Err(ExecuteError::ret_addr_stack_is_empty()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fun_initialization() {
        let mut context = Environment::default();
        let inst = FunInst::new(Operand::Label("test".to_string()));
        inst.initialize(&mut context, 0).unwrap();
        assert_eq!(context.fun_labels.get("test"), Some(&0));
    }

    #[test]
    fn test_pusha() {
        let mut context = Environment::default();
        let mut inst = PushaInst::new(Operand::Constant(10));
        inst.execute(&mut context).unwrap();
        assert_eq!(context.arg_queue.pop_front(), Some(10));
    }

    #[test]
    fn test_popa() {
        let mut context = Environment::default();
        let mut inst = PushaInst::new(Operand::Constant(10));
        inst.execute(&mut context).unwrap();

        let mut inst = PopaInst::new(Operand::Variable("test".to_string()));
        inst.execute(&mut context).unwrap();
        let val = inst.dest.value(&context).unwrap();
        assert_eq!(val, 10);
    }

    #[test]
    fn test_pushr() {
        let mut context = Environment::default();
        let mut inst = PushrInst::new(Operand::Constant(10));
        inst.execute(&mut context).unwrap();
        assert_eq!(context.ret_queue.pop_front(), Some(10));
    }

    #[test]
    fn test_popr() {
        let mut context = Environment::default();
        let mut inst = PushrInst::new(Operand::Constant(10));
        inst.execute(&mut context).unwrap();

        let mut inst = PoprInst::new(Operand::Variable("test".to_string()));
        inst.execute(&mut context).unwrap();
        let val = inst.dest.value(&context).unwrap();
        assert_eq!(val, 10);
    }

    #[test]
    fn test_call() {
        let mut context = Environment::default();

        let inst = FunInst::new(Operand::Label("test".to_string()));
        inst.initialize(&mut context, 0).unwrap();
        inst.inc_inst_ptr(&mut context).unwrap();

        let mut inst = CallInst::new(Operand::Label("test".to_string()));
        inst.execute(&mut context).unwrap();
        inst.inc_inst_ptr(&mut context).unwrap();
        assert_eq!(context.ret_addr_stack.pop(), Some(2));
        assert_eq!(context.inst_ptr, 0);
    }

    #[test]
    fn test_ret() {
        let mut context = Environment::default();

        let inst = FunInst::new(Operand::Label("test".to_string()));
        inst.initialize(&mut context, 0).unwrap();
        inst.inc_inst_ptr(&mut context).unwrap();

        let mut inst = CallInst::new(Operand::Label("test".to_string()));
        inst.execute(&mut context).unwrap();
        inst.inc_inst_ptr(&mut context).unwrap();

        let mut inst = RetInst::new();
        inst.execute(&mut context).unwrap();
        inst.inc_inst_ptr(&mut context).unwrap();
        assert_eq!(context.inst_ptr, 2);
    }
}