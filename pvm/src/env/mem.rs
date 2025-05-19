use std::collections::{BTreeMap, HashMap};
use crate::env::MemoryError;
use crate::Error;

const WORD_SIZE: usize = size_of::<u64>();

/// The execution memory.
#[derive(Debug)]
pub(crate) struct Memory {
    bytes: Box<[u8]>,
    in_use: HashMap<usize, usize>,
    free: BTreeMap<usize, usize>,
}

impl Memory {
    /// Creates a new memory.
    pub(crate) fn new(size: usize) -> Self {
        Self {
            bytes: vec![0; size].into_boxed_slice(),
            in_use: HashMap::new(),
            free: BTreeMap::from([(0, size)]),
        }
    }

    /// Allocates a block of memory.
    pub(crate) fn allocate(&mut self, size: usize) -> Result<usize, Error> {
        if size == 0 {
            return Err(MemoryError::cannot_allocate_zero_bytes());
        }

        let mut addr = None::<usize>;
        for (i, s) in &self.free {
            if *s >= size {
                addr = Some(*i);
                break;
            }
        }

        match addr {
            Some(a) => {
                let s = self.free.remove(&a).unwrap();
                let b_size = s - size;
                if b_size > 0 {
                    let b = a + size;
                    self.free.insert(b, b_size);
                }

                self.in_use.insert(a, size);
                Ok(a)
            }
            None => Err(MemoryError::out_of_memory()),
        }
    }

    /// Deallocates a block of memory.
    pub(crate) fn deallocate(&mut self, addr: usize) -> Result<(), Error> {
        if addr >= self.bytes.len() {
            return Err(MemoryError::address_out_of_bounds());
        }

        let size = match self.in_use.remove(&addr) {
            Some(s) => s,
            None => return Ok(()),
        };

        // defragment memory
        let mut before: Option<(usize, usize)> = None;
        let mut after: Option<(usize, usize)> = None;
        for (i, s) in &self.free {
            if i + s == addr {
                before = Some((*i, *s));
            }

            if *i == addr + size {
                after = Some((*i, *s));
            }
        }
        
        if let (Some((b, b_size)), Some((a, a_size))) = (before, after) {
            self.free.remove(&a);
            self.free.insert(b, b_size + size + a_size);
        } else if let Some((b, b_size)) = before {
            self.free.insert(b, b_size + size);
        } else if let Some((a, a_size)) = after {
            self.free.remove(&a);
            self.free.insert(addr, size + a_size);
        } else {
            self.free.insert(addr, size);
        }
        
        Ok(())
    }
    
    /// Gets the length of an allocation in bytes. 
    /// `addr` must point to a valid allocation.
    pub(crate) fn len(&self, addr: usize) -> Result<usize, Error> {
        match self.in_use.get(&addr) {
            Some(s) => Ok(*s),
            None => Err(MemoryError::invalid_address()),
        }
    }

    /// Loads a value from memory.
    pub(crate) fn load(&self, addr: usize) -> Result<u64, Error> {
        if addr + WORD_SIZE - 1 >= self.bytes.len() {
            Err(MemoryError::address_out_of_bounds())
        } else {
            let bytes: [u8; WORD_SIZE] = match self.bytes[addr..addr + WORD_SIZE].try_into() {
                Ok(bytes) => bytes,
                Err(_) => return Err(MemoryError::invalid_address()),
            };
            Ok(u64::from_be_bytes(bytes))
        }
    }

    /// Stores a value into memory.
    pub(crate) fn store(&mut self, addr: usize, word: u64) -> Result<(), Error> {
        if addr + WORD_SIZE - 1 >= self.bytes.len() {
            Err(MemoryError::address_out_of_bounds())
        } else {
            let bytes: [u8; WORD_SIZE] = word.to_be_bytes();
            for i in 0..WORD_SIZE {
                self.bytes[addr + i] = bytes[i];
            }

            Ok(())
        }
    }

    /// Loads a byte from memory.
    pub(crate) fn load_byte(&self, addr: usize) -> Result<u8, Error> {
        if addr >= self.bytes.len() {
            Err(MemoryError::address_out_of_bounds())
        } else {
            Ok(self.bytes[addr])
        }
    }

    /// Stores a byte into memory.
    pub(crate) fn store_byte(&mut self, addr: usize, byte: u8) -> Result<(), Error> {
        if addr >= self.bytes.len() {
            Err(MemoryError::address_out_of_bounds())
        } else {
            self.bytes[addr] = byte;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let mem = Memory::new(32);
        let v = mem.load(0).unwrap();
        assert_eq!(v, 0);
    }

    #[test]
    fn test_store() {
        let mut mem = Memory::new(32);
        let v = 1000*1000;
        mem.store(0, v).unwrap();
        assert_eq!(mem.load(0).unwrap(), v);
    }

    #[test]
    #[should_panic]
    fn test_load_fails() {
        let mem = Memory::new(32);
        let _ = mem.load(30).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_store_fails() {
        let mut mem = Memory::new(32);
        mem.store(30, 0).unwrap();
    }

    #[test]
    fn test_load_byte() {
        let mem = Memory::new(32);
        let v = mem.load_byte(0).unwrap();
        assert_eq!(v, 0);
    }

    #[test]
    fn test_store_byte() {
        let mut mem = Memory::new(32);
        let v = 100;
        mem.store_byte(0, v).unwrap();
        assert_eq!(mem.load_byte(0).unwrap(), v);
    }

    #[test]
    #[should_panic]
    fn test_load_byte_fails() {
        let mem = Memory::new(32);
        let _ = mem.load_byte(32).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_store_byte_fails() {
        let mut mem = Memory::new(32);
        mem.store_byte(32, 0).unwrap();
    }

    #[test]
    fn test_allocate() {
        const SIZE: usize = 32;

        let allocations = vec![
            vec![SIZE],
            vec![10, 10, 10, 2],
            vec![10, 10, 2],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![2, 2, 3, 8, 2],
        ];

        for sizes in allocations {
            let mut mem = Memory::new(SIZE);
            for s in &sizes {
                let a = mem.allocate(*s).unwrap();
                assert!(a < SIZE);

                mem.store_byte(a, 0).unwrap();
                let v = mem.load_byte(a).unwrap();
                assert_eq!(v, 0);
            }

            assert_eq!(mem.in_use.len(), sizes.len());
            if sizes.iter().sum::<usize>() == SIZE {
                assert!(mem.free.is_empty());
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_allocate_too_much_fails() {
        const SIZE: usize = 32;
        let mut mem = Memory::new(SIZE);
        let _ = mem.allocate(SIZE+1).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_allocate_zero_fails() {
        let mut mem = Memory::new(32);
        let _ = mem.allocate(0).unwrap();
    }

    #[test]
    fn test_deallocate() {
        const SIZE: usize = 32;

        let allocations = vec![
            vec![SIZE],
            vec![10, 10, 10, 2],
            vec![10, 10, 2],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![2, 2, 3, 8, 2],
        ];

        for sizes in allocations {
            let mut mem = Memory::new(SIZE);
            let mut addrs = vec![];
            for s in &sizes {
                let a = mem.allocate(*s).unwrap();
                assert!(a < SIZE);

                addrs.push(a);
            }

            for a in addrs {
                mem.deallocate(a).unwrap()
            }
            
            assert_eq!(mem.free.len(), 1);
            assert_eq!(*mem.free.get(&0).unwrap(), SIZE);
            assert!(mem.in_use.is_empty());
        }
    }
}
