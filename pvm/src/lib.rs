mod conf;
mod env;
mod inst;
mod operand;
mod parse;
mod cast;
mod error;

pub use crate::conf::ExecuteConfig;
pub use crate::inst::*;
pub use crate::parse::*;
pub use crate::error::*;

use crate::env::Environment;

/// Executes the PVM instructions with the default configuration.
pub fn execute(instructions: Vec<Box<dyn Instruction>>)  -> Result<(), Error> {
    let config = ExecuteConfig::default();
    execute_with_config(instructions, config)
}

/// Executes the PVM instructions with the provided configuration.
pub fn execute_with_config(mut instructions: Vec<Box<dyn Instruction>>, config: ExecuteConfig) -> Result<(), Error> {
    let mut env = Environment::new(config.memory_size, config.stdout);

    // validation pass
    for (i, instruction) in instructions.iter().enumerate() {
        let result = instruction.validate();
        wrap(result, i+1)?
    }

    // initialization pass
    for (i, instruction) in instructions.iter().enumerate() {
        let result = instruction.initialize(&mut env, i);
        wrap(result, i+1)?
    }

    // execute loop
    loop {
        if env.inst_ptr >= instructions.len() {
            break;
        }

        // fetch the current instruction
        let inst = &mut instructions[env.inst_ptr];

        // execute the instruction
        let result = inst.execute(&mut env);
        wrap(result, env.inst_ptr+1)?;

        // increment the instruction pointer
        let result =  inst.inc_inst_ptr(&mut env);
        wrap(result, env.inst_ptr+1)?;
    }
    
    Ok(())
}
