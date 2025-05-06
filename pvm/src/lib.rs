mod inst;
mod env;
mod conf;

use crate::conf::ExecuteConfig;
use crate::inst::*;
use crate::env::Environment;

pub fn execute(instructions: Vec<Box<dyn Execute>>) {
    let config = ExecuteConfig::default();
    execute_with_config(instructions, config);
}

pub fn execute_with_config(instructions: Vec<Box<dyn Execute>>, config: ExecuteConfig) {
    let mut context = Environment::new(config.memory_size);
    for instruction in instructions {
        instruction.execute(&mut context);
    }
}

