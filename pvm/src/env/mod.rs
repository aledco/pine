mod mem;

use std::cell::RefCell;
use crate::env::mem::Memory;
use std::collections::HashMap;
use std::io::{Write};
use std::rc::Rc;

pub struct Environment {
    //pub memory: Memory,
    pub variables: HashMap<String, u64>,
    pub labels: HashMap<String, usize>,
    pub global_strings: HashMap<String, String>, // TODO do global constants instead?\
    pub stdout: Rc<RefCell<dyn Write>>,
    pub inst_ptr: usize,
}

impl Environment {
    pub fn new(memory_size: usize, stdout:  Rc<RefCell<dyn Write>>) -> Self {
        Self {
            //memory: Memory::new(memory_size),
            variables: HashMap::new(),
            labels: HashMap::new(),
            global_strings: HashMap::new(),
            stdout,
            inst_ptr: 0,
        }
    }
}
