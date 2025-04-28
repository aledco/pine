

pub enum SymbolType {
    Variable,
    Parameter,
    Function,
} 

pub struct Symbol<'a> {
    pub name: String,
    pub symbol_type: SymbolType,
    pub scope: &'a Scope,
}

pub struct SymbolTable {
    
}

impl SymbolTable {
    
}

pub struct Scope {
    
}