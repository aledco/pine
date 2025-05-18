mod mem;
mod error;

pub(crate) use error::*;
use std::cell::RefCell;
use crate::env::mem::Memory;
use std::collections::HashMap;
use std::io::{Write};
use std::rc::Rc;

pub struct Environment {
    pub memory: Memory,
    pub variables: HashMap<String, u64>,
    pub labels: HashMap<String, usize>,
    pub fun_labels: HashMap<String, usize>,
    pub arg_stack: Vec<u64>,
    pub ret_val: u64,
    pub stdout: Rc<RefCell<dyn Write>>,
    pub inst_ptr: usize,
}

impl Environment {
    pub fn new(memory_size: usize, stdout:  Rc<RefCell<dyn Write>>) -> Self {
        Self {
            memory: Memory::new(memory_size),
            variables: HashMap::new(),
            labels: HashMap::new(),
            fun_labels: HashMap::new(),
            arg_stack: Vec::new(),
            ret_val: 0,
            stdout,
            inst_ptr: 0,
        }
    }
}
