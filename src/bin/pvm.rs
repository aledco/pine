use std::fs;
use pvm::{parse, execute};

fn main() {
    let input = fs::read_to_string("/tmp/test.pvm").unwrap();
    let instructions = parse(&input).unwrap();
    execute(instructions);
}
