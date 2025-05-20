use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("input file not provided");

    let input = fs::read_to_string(input_file).unwrap();

    let mut program = ast::parse(input).unwrap(); // TODO handle error
    sem::traverse(&mut program);

    // TODO code gen
}
