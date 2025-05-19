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
    pub(crate) variables: HashMap<String, u64>,
    pub(crate) labels: HashMap<String, usize>,
    pub(crate) fun_labels: HashMap<String, usize>,
    pub(crate) arg_queue: VecDeque<u64>,
    pub(crate) ret_queue: VecDeque<u64>,
    pub(crate) ret_addr_stack: Vec<usize>,
    pub(crate) local_var_store: HashMap<String, Vec<u64>>,
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
            variables: HashMap::new(),
            labels: HashMap::new(),
            fun_labels: HashMap::new(),
            arg_queue: VecDeque::new(),
            ret_queue: VecDeque::new(),
            ret_addr_stack: Vec::new(),
            local_var_store: HashMap::new(),
            stdin,
            stdout,
            inst_ptr: 0,
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
