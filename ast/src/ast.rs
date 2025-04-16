pub struct Expression {
    pub test: String,
}

impl Expression {
    pub fn new() -> Expression {
        Expression { test: String::from("test") }
    } 
}