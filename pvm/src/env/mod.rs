mod mem;

use crate::env::mem::Memory;
use std::collections::HashMap;

pub struct Environment {
    pub memory: Memory,
    pub labels: HashMap<String, usize>,
    pub global_strings: HashMap<String, String>, // TODO do global constants instead?
    //pub stack_ptr: usize,
    //pub frame_ptr: usize,
    pub inst_ptr: usize,
}

impl Environment {
    pub fn new(memory_size: usize) -> Self {
        Self {
            memory: Memory::new(memory_size),
            labels: HashMap::new(),
            global_strings: HashMap::new(),
            //stack_ptr: memory_size-1,
            //frame_ptr: memory_size-1,
            inst_ptr: 0,
        }
    }
}
