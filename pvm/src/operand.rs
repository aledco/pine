#[derive(Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Address(usize),
}

#[derive(Debug)]
pub enum Operand {
    Constant(Value),
    Variable(String, Value),
    Label(String),
}
