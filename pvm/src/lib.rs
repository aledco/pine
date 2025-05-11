mod conf;
mod env;
mod inst;
mod operand;
mod parse;

pub use crate::conf::ExecuteConfig;
pub use crate::inst::*;
pub use crate::parse::*;

use crate::env::Environment;

pub fn execute(instructions: Vec<Box<dyn Instruction>>)  -> Result<(), String> {
    let config = ExecuteConfig::default();
    execute_with_config(instructions, config)
}

pub fn execute_with_config(mut instructions: Vec<Box<dyn Instruction>>, config: ExecuteConfig) -> Result<(), String> {
    let mut context = Environment::new(config.memory_size, config.stdout);

    // initial pass
    for (i, instruction) in instructions.iter().enumerate() {
        if let Some(label) = instruction.defined_label() {
            context.labels.insert(label, (i+1) as u64);
        }

        let used_vars = instruction.used_vars();
        for var in used_vars {
            let name = var.var_name()?;
            if !context.variables.contains_key(&name) {
                panic!("Variable {} was never defined", name);
            }
        }

        if let Some(var) = instruction.defined_var() {
            let name = var.var_name()?;
            context.variables.insert(name, 0);
        }
    }

    // execute loop
    loop {
        if context.inst_ptr >= instructions.len() {
            break; // TODO use end inst instead?
        }

        let inst = &mut instructions[context.inst_ptr];
        inst.execute(&mut context)?;
        inst.inc_inst_ptr(&mut context)?;
    }
    
    Ok(())
}
