use std::cell::RefCell;
use std::collections::HashMap;

pub enum SymbolType {
    Variable,
    Parameter,
    Function,
} 

pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
}

pub struct SymbolTable {
    symbols: HashMap<String, Symbol>
}

pub enum ScopeLevel {
    Global,
    Local(usize)
}

pub struct Scope {
    pub parent: Option<RefCell<Box<Scope>>>,
    pub symbol_table: SymbolTable,
    pub level: ScopeLevel,
}
    
impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType) -> Self {
        Self {
            name,
            symbol_type,
        }
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }
    
    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }
    
    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn add(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }
}

impl Scope {
    pub fn new_global() -> Scope {
        Self {
            parent: None,
            symbol_table: SymbolTable::new(),
            level: ScopeLevel::Global
        }
    }
    
    pub fn new_local(parent: Box<Scope>) -> Scope {
        let level = match parent.level {
            ScopeLevel::Global => ScopeLevel::Local(0),
            ScopeLevel::Local(depth) => ScopeLevel::Local(depth + 1)
        };
        
        Self {
            parent: Some(RefCell::new(parent)),
            symbol_table: SymbolTable::new(),
            level
        }
    }
    
    pub fn add(&mut self, symbol: Symbol) {
        self.symbol_table.add(symbol);
    }
    
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        if let Some(symbol) = self.symbol_table.get(name) {
            Some(symbol)
        } else if let Some(parent) = &self.parent {
            parent.lookup(name)
        } else {
            None
        }
    }
}