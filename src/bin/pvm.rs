use std::fmt::Display;
use std::fs;
use pvm::{parse, execute};

fn main() {
    let input = fs::read_to_string("/tmp/test.pvm").unwrap();

    let instructions_result = parse(&input);
    let instructions = handle_result(instructions_result);

    let execute_result = execute(instructions);
    handle_result(execute_result);
}

fn handle_result<T, E>(result: Result<T, E>) -> T
where E : Display {
    match result {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
    }
}
