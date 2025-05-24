mod error;
mod mem;

use crate::env::mem::Memory;
pub(crate) use error::*;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::rc::Rc;

/// The execution environment.
pub struct Environment {
    pub(crate) memory: Memory,
    global_variables: HashMap<String, u64>,
    pub(crate) labels: HashMap<String, usize>,
    pub(crate) fun_labels: HashMap<String, usize>,
    fun_variable_stack: Vec<HashMap<String, u64>>,
    pub(crate) arg_queue: VecDeque<u64>,
    pub(crate) ret_queue: VecDeque<u64>,
    pub(crate) ret_addr_stack: Vec<usize>,
    pub(crate) stdin: Rc<RefCell<dyn Read>>,
    pub(crate) stdout: Rc<RefCell<dyn Write>>,
    pub(crate) inst_ptr: usize,
}

impl Environment {
    /// Creates a new environment.
    pub(crate) fn new(
        memory_size: usize,
        stdin: Rc<RefCell<dyn Read>>,
        stdout: Rc<RefCell<dyn Write>>,
    ) -> Self {
        Self {
            memory: Memory::new(memory_size),
            global_variables: HashMap::new(),
            labels: HashMap::new(),
            fun_labels: HashMap::new(),
            fun_variable_stack: Vec::new(),
            arg_queue: VecDeque::new(),
            ret_queue: VecDeque::new(),
            ret_addr_stack: Vec::new(),
            stdin,
            stdout,
            inst_ptr: 0
        }
    }
    
    pub(crate) fn push_variable_stack(&mut self) {
        self.fun_variable_stack.push(HashMap::new());
    }
    
    pub(crate) fn pop_variable_stack(&mut self) {
        self.fun_variable_stack.pop();
    }
    
    pub(crate) fn variable(&self, name: &str) -> Option<u64> {
        match self.fun_variable_stack.last() {
            Some(variables) => {
                match variables.get(name) {
                    Some(value) => Some(*value),
                    None => self.global_variables.get(name).copied()
                }
            },
            None => {
                self.global_variables.get(name).copied()
            }
        }
    }

    pub(crate) fn set_variable(&mut self, name: &str, value: u64) {
        match self.fun_variable_stack.last_mut() {
            Some(variables) => {
                if variables.contains_key(name) || !self.global_variables.contains_key(name) {
                    variables.insert(name.to_string(), value);
                } else {
                    self.global_variables.insert(name.to_string(), value);
                }
            },
            None => {
                self.global_variables.insert(name.to_string(), value);
            }
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new(
            1024,
            Rc::new(RefCell::new(std::io::stdin())),
            Rc::new(RefCell::new(std::io::stdout())),
        )
    }
}
