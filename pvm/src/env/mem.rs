pub struct Memory {
    words: Box<[u64]>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            words: vec![0; size].into_boxed_slice(),
        }
    }
}
