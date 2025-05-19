use crate::ast::PineType;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type SymbolRef = Rc<RefCell<Symbol>>;
pub type ScopeRef = Rc<RefCell<Scope>>;

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub pine_type: PineType,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, SymbolRef>,
}

#[derive(Debug, PartialEq)]
pub enum ScopeDepth {
    Global,
    Local(usize),
}

#[derive(Debug)]
pub struct Scope {
    pub parent: Option<ScopeRef>,
    pub symbol_table: SymbolTable,
    pub depth: ScopeDepth,
}

impl Symbol {
    pub fn default() -> SymbolRef {
        Rc::new(RefCell::new(Self {
            name: String::default(),
            pine_type: PineType::Unknown,
        }))
    }

    pub fn new(name: String) -> SymbolRef {
        Rc::new(RefCell::new(Self {
            name,
            pine_type: PineType::Unknown,
        }))
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

    pub fn add(&mut self, symbol: SymbolRef) -> Result<(), ()> {
        let name = symbol.borrow().name.clone();
        if self.symbols.contains_key(&name) {
            Err(())
        } else {
            self.symbols.insert(name, symbol);
            Ok(())
        }
    }
}

impl Scope {
    pub fn default() -> ScopeRef {
        Rc::new(RefCell::new(Self {
            parent: None,
            symbol_table: SymbolTable::new(),
            depth: ScopeDepth::Global,
        }))
    }

    pub fn new_global() -> ScopeRef {
        Rc::new(RefCell::new(Self {
            parent: None,
            symbol_table: SymbolTable::new(),
            depth: ScopeDepth::Global,
        }))
    }

    pub fn new_local(parent: ScopeRef) -> ScopeRef {
        let level = match parent.borrow().depth {
            ScopeDepth::Global => ScopeDepth::Local(1),
            ScopeDepth::Local(depth) => ScopeDepth::Local(depth + 1),
        };

        Rc::new(RefCell::new(Self {
            parent: Some(parent),
            symbol_table: SymbolTable::new(),
            depth: level,
        }))
    }

    pub fn add(&mut self, symbol: SymbolRef) -> Result<(), ()> {
        self.symbol_table.add(symbol)
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
