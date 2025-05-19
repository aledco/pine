use std::cell::RefCell;
use std::io::{stdin, stdout, Read, Write};
use std::rc::Rc;

const DEFAULT_MEMORY_SIZE: usize = 1024 * 1024;

/// The execution configuration.
pub struct ExecuteConfig {
    pub memory_size: usize,
    pub stdin: Rc<RefCell<dyn Read>>,
    pub stdout: Rc<RefCell<dyn Write>>,
}

impl Default for ExecuteConfig {
    fn default() -> Self {
        Self {
            memory_size: DEFAULT_MEMORY_SIZE,
            stdin: Rc::new(RefCell::new(stdin())),
            stdout: Rc::new(RefCell::new(stdout())),
        }
    }
}

impl ExecuteConfig {
    pub fn new(
        memory_size: usize,
        stdin: Rc<RefCell<dyn Read>>,
        stdout: Rc<RefCell<dyn Write>>,
    ) -> Self {
        Self {
            memory_size,
            stdin,
            stdout,
        }
    }
}
