mod mem;
mod error;

pub(crate) use error::*;
use std::cell::RefCell;
use crate::env::mem::Memory;
use std::collections::{HashMap, VecDeque};
use std::io::{Write};
use std::rc::Rc;

pub struct Environment {
    pub memory: Memory,
    pub variables: HashMap<String, u64>,
    pub labels: HashMap<String, usize>,
    pub fun_labels: HashMap<String, usize>,
    pub arg_queue: VecDeque<u64>,
    pub ret_queue: VecDeque<u64>,
    pub ret_addr_stack: Vec<usize>,
    pub local_var_store: HashMap<String, Vec<u64>>,
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
            arg_queue: VecDeque::new(),
            ret_queue: VecDeque::new(),
            ret_addr_stack: Vec::new(),
            local_var_store: HashMap::new(),
            stdout,
            inst_ptr: 0,
        }
    }
}
