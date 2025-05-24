use std::collections::HashSet;

pub(crate) struct TempStore { 
    pub in_use: HashSet<String>,
    pub free: Vec<String>,
    pub counter: usize,
}

impl TempStore {
    pub fn new() -> Self {
        Self {
            in_use: HashSet::new(),
            free: Vec::new(),
            counter: 0,
        }
    }
    
    pub fn temp(&mut self) -> String {
        match self.free.pop() {
            Some(t) => t,
            None => {
                let t = format!("t{}", self.counter);
                self.counter += 1;
                self.in_use.insert(t.clone());
                t
            }
        }
    }
    
    pub fn free(&mut self, t: String) {
        self.in_use.remove(&t);
        self.free.push(t);
    }
}