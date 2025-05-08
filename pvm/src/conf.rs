const DEFAULT_MEMORY_SIZE: usize = 1024 * 1024;

pub struct ExecuteConfig {
    pub memory_size: usize,
}

impl Default for ExecuteConfig {
    fn default() -> Self {
        Self { memory_size: DEFAULT_MEMORY_SIZE }
    }
}

impl ExecuteConfig {
    pub fn new(memory_size: usize) -> Self {
        Self { memory_size }
    }
}
