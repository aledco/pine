pub(crate) struct LabelGen {
    counter: usize,
}

impl LabelGen {
    fn new() -> Self {
        Self { counter: 0 }
    }
    
    fn label(&mut self, s: String) -> String {
        self.counter += 1;
        s
    }
    
    pub(crate) fn if_prefix(&mut self) -> String {
        self.label(format!("if{}", self.counter))
    }

    pub(crate) fn while_prefix(&mut self) -> String {
        self.label(format!("while{}", self.counter))
    }
}

pub(crate) struct Context {
    pub label_gen: LabelGen
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            label_gen: LabelGen::new()
        }
    }
}