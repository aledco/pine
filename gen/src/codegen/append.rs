use crate::codegen::{Inst, InstVec};

pub(crate) trait Append<T> {
    fn append(self, value: T) -> InstVec;
}

impl Append<Inst> for InstVec {
    fn append(mut self, value: Inst) -> InstVec {
        self.push(value);
        self
    }
}

impl Append<InstVec> for InstVec {
    fn append(mut self, value: InstVec) -> InstVec {
        self.extend(value);
        self
    }
}


impl Append<InstVec> for Inst {
    fn append(self, mut value: InstVec) -> InstVec {
        value.insert(0, self);
        value
    }
}

impl Append<Inst> for Inst {
    fn append(self, value: Inst) -> InstVec {
        vec![self, value]
    }
}
