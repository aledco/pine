pub struct Memory {
    words: Box<[u64]>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            words: vec![0; size].into_boxed_slice(),
        }
    }
    
    pub fn get(&self, i: usize) -> u64 {
        self.words[i]
    }
    
    pub fn set(&mut self, i: usize, value: u64) {
        self.words[i] = value;
    }
}
