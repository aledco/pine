use std::fmt::Display;
use std::fs;
use std::env;
use pvm::{parse, execute};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("input file not provided");
    
    let input = fs::read_to_string(input_file).unwrap();

    let instructions_result = parse(&input);
    let instructions = handle_result(instructions_result);

    let execute_result = execute(instructions);
    if let Err(pvm::Error::ExitError(e)) = execute_result {
        eprintln!("{}", e);
        std::process::exit(e.exit_code);
    }
    
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
