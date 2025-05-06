mod reg;
mod mem;

use crate::env::mem::Memory;
use crate::env::reg::Register;

pub struct Environment {
    pub memory: Memory,
    //pub registers: [Register; 8],
}

impl Environment {
    pub fn new(memory_size: usize) -> Self {
        Self {
            memory: Memory::new(memory_size),
           // registers: [],
        }
    }
}