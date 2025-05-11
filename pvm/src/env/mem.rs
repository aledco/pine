const WORD_SIZE: usize = size_of::<u64>();

pub struct Memory {
    words: Box<[u8]>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            words: vec![0; size].into_boxed_slice(),
        }
    }

    pub fn get_word(&self, i: usize) -> Result<u64, String> {
        if i % WORD_SIZE != 0 {
            return Err("alignment error".to_string());
        }

        let bytes: [u8; WORD_SIZE] = self.words[i..i + WORD_SIZE].try_into().unwrap();
        let word = u64::from_ne_bytes(bytes);
        Ok(word)
    }

    pub fn set_word(&mut self, i: usize, word: u64) -> Result<(), String> {
        if i % WORD_SIZE != 0 {
            return Err("alignment error".to_string());
        }
        
        let bytes: [u8; WORD_SIZE] = word.to_ne_bytes();
        for j in 0..WORD_SIZE {
            self.words[i+j] = bytes[j];
        }

        Ok(())
    }
}
