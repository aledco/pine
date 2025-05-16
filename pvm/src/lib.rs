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

pub fn execute(instructions: Vec<Box<dyn Instruction>>)  -> Result<(), Error> {
    let config = ExecuteConfig::default();
    execute_with_config(instructions, config)
}

pub fn execute_with_config(mut instructions: Vec<Box<dyn Instruction>>, config: ExecuteConfig) -> Result<(), Error> {
    let mut context = Environment::new(config.memory_size, config.stdout);

    // validation pass
    for instruction in &instructions {
        instruction.validate()?;
    }
    
    // initialization pass
    for (i, instruction) in instructions.iter().enumerate() {
        instruction.initialize(&mut context, i)?;
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
