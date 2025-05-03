use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

pub type SymbolRef = Rc<RefCell<Symbol>>;
pub type ScopeRef = Rc<RefCell<Scope>>;

// pub enum SymbolType {
//     Variable,
//     Parameter,
//     Function,
// }

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    //pub symbol_type: SymbolType,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, SymbolRef>,
}

#[derive(Debug)]
pub enum ScopeLevel {
    Global,
    Local(usize),
}

#[derive(Debug)]
pub struct Scope {
    pub parent: Option<ScopeRef>,
    pub symbol_table: SymbolTable,
    pub level: ScopeLevel,
}

impl Symbol {
    pub fn new(name: String) -> SymbolRef {
        Rc::new(RefCell::new(Self { name }))
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

    pub fn get(&self, name: &str) -> Option<SymbolRef> {
        match self.symbols.get(name) {
            Some(sym) => Some(Rc::clone(sym)),
            None => None,
        }
    }

    pub fn add(&mut self, symbol: SymbolRef) {
        let name = symbol.borrow().name.clone();
        self.symbols.insert(name, symbol);
    }
}

impl Scope {
    pub fn new_global() -> ScopeRef {
        Rc::new(RefCell::new(Self {
            parent: None,
            symbol_table: SymbolTable::new(),
            level: ScopeLevel::Global,
        }))
    }

    pub fn new_local(parent: ScopeRef) -> ScopeRef {
        let level = match parent.borrow().level {
            ScopeLevel::Global => ScopeLevel::Local(0),
            ScopeLevel::Local(depth) => ScopeLevel::Local(depth + 1),
        };

        Rc::new(RefCell::new(Self {
            parent: Some(parent),
            symbol_table: SymbolTable::new(),
            level,
        }))
    }

    pub fn add(&mut self, symbol: SymbolRef) {
        self.symbol_table.add(symbol);
    }

    pub fn lookup(&self, name: &str) -> Option<SymbolRef> {
        match self.symbol_table.get(name) {
            Some(symbol) => Some(symbol),
            None => match &self.parent {
                Some(parent) => parent.borrow().lookup(name),
                None => None,
            },
        }
    }
}
