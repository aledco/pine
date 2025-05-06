pub struct Register {
    pub name: String,
    pub index: usize,
}

impl Register {
    pub fn new(name: String) -> Self {
        Self { name, index: 0 }
    }

    // pub fn all() -> [Self] {
    //
    // }
}