mod mem;

use std::cell::RefCell;
use crate::env::mem::Memory;
use std::collections::HashMap;
use std::io::{Write};
use std::rc::Rc;

pub struct Environment {
    pub memory: Memory,
    pub variables: HashMap<String, u64>,
    pub labels: HashMap<String, u64>,
    pub global_strings: HashMap<String, String>, // TODO do global constants instead?\
    pub stdout: Rc<RefCell<dyn Write>>,
    //pub stack_ptr: usize,
    //pub frame_ptr: usize,
    pub inst_ptr: usize,
}

impl Environment {
    pub fn new(memory_size: usize) -> Self {
        Self {
            memory: Memory::new(memory_size),
            variables: HashMap::new(),
            labels: HashMap::new(),
            global_strings: HashMap::new(),
            stdout: Rc::new(RefCell::new(std::io::stdout())),
            //stack_ptr: memory_size-1,
            //frame_ptr: memory_size-1,
            inst_ptr: 0,
        }
    }

    pub fn new_with_stdout(memory_size: usize, stdout:  Rc<RefCell<dyn Write>>) -> Self {
        Self {
            memory: Memory::new(memory_size),
            variables: HashMap::new(),
            labels: HashMap::new(),
            global_strings: HashMap::new(),
            stdout,
            //stack_ptr: memory_size-1,
            //frame_ptr: memory_size-1,
            inst_ptr: 0,
        }
    }
}
