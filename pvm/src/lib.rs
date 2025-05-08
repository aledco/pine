mod conf;
mod env;
mod inst;
mod operand;

use crate::conf::ExecuteConfig;
use crate::env::Environment;
use crate::inst::*;

pub fn execute(instructions: Vec<Box<dyn Instruction>>) {
    let config = ExecuteConfig::default();
    execute_with_config(instructions, config);
}

pub fn execute_with_config(instructions: Vec<Box<dyn Instruction>>, config: ExecuteConfig) {
    let mut context = Environment::new(config.memory_size);
    loop {
        if context.inst_ptr >= instructions.len() {
            break; // TODO use end inst instead?
        }

        let inst = &instructions[context.inst_ptr];
        inst.execute(&mut context);
        inst.inc_inst_ptr(&mut context);
    }
}
