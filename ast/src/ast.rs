pub trait AST {
    
}

pub struct Program {
    pub nodes: Vec<Box<dyn AST>>,
    pub main: FunctionDef,
}

pub struct FunctionDef {
    
}

pub struct Expression {
}


impl AST for FunctionDef {
    
}
