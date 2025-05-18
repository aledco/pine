use pvm_proc_macros::Inst;
use crate::env::Environment;
use crate::inst::*;
use crate::operand::*;
use crate::parse::{Line, Literal, Parse, Token};

extern crate pvm_proc_macros;
use pvm_proc_macros::*;

// TODO write tests

#[inst(name = "fun", operands = [OperandFormat::Label])]
pub struct FunInst {
    pub(crate) lab: Operand,
}

impl Instruction for FunInst {
    fn execute(&mut self, _env: &mut Environment) -> Result<(), Error> {
        Ok(())
    }

    fn initialize(&self, env: &mut Environment, i: usize) -> Result<(), Error> {
        let label = self.lab.label()?;
        if env.fun_labels.contains_key(&label) {
            return Err(ValidateError::label_already_defined(&label));
        }

        env.fun_labels.insert(label, i+1);
        Ok(())
    }
}

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

#[inst(name = "ret", operands = [])]
pub struct RetInst {
}

impl Instruction for RetInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        env.arg_queue.clear();
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

