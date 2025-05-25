use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use crate::ast::*;
use crate::sem::{SemError, SemResult};

pub(crate) fn resolve_modules(main_module: &Module) -> SemResult<Vec<Module>> {
    // TODO produce graph of module references

    let modules: Vec<ModuleNode> = main_module.imports
        .iter()
        .map(|i| ModuleNode::new(i.ident.name.clone()))
        .collect();



    // TODO analyze graph for cycles

    // TODO parse unique modules

    // TODO produce list of modules to be processed in order

    todo!()
}

struct ModuleNode {
    name: String,
    modules: Vec<ModuleNode>,
}

impl ModuleNode {
    pub fn new(name: String) -> Self {
        Self {
            name,
            modules: vec![],
        }
    }
}

struct ModuleTraverser {
    pub curr_path: PathBuf,
    pub modules: HashMap<String, Module>,
    pub dependency_graph: ModuleNode
}

impl ModuleTraverser {
    pub fn new(root_path: &str) -> Self {
        Self {
            curr_path: PathBuf::from(root_path),
            modules: HashMap::new(),
            dependency_graph: ModuleNode::new("root".to_string())
        }
    }
    
    pub fn traverse(&mut self, module: &Module) -> SemResult<()> {
        for import in &module.imports {
            let mut path = self.curr_path.join(import.ident.name.clone());
            if path.exists() {
                path.push("mod.p");
            } else {
                path.set_extension("p");
            }
            
            if !path.exists() {
                return Err(SemError::error("module does not exist", import.span()));
            }
            
            let key = match path.to_str() {
                Some(s) => s.to_string(),
                None => return Err(SemError::error("module does not exist", import.span()))
            };
            
            if self.modules.contains_key(&key) {
                // TODO traverse imports tp build graph
            } else {
                // TODO parse module and add to modules
                // TODO traverse imports tp build graph
            }
        }

        Ok(())
    } 
}
