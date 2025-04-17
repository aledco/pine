pub trait AST {
    
}

pub struct Program {
    pub modules: Vec<Module>,
    pub main: FunctionDef,
}

pub struct Module {
    pub functions: Vec<FunctionDef>,
}

pub struct FunctionDef {
    
}

pub struct Expression {
}


impl AST for FunctionDef {
    
}
