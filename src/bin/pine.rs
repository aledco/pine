use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("input file not provided");
    //let output_file = args.get(2).expect("output file not provided");

    let input = fs::read_to_string(input_file).unwrap();

    let mut program = ast::parse(input).unwrap(); // TODO handle error

    let instructions = gen::codegen(&mut program);

    let output = instructions
        .iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", output);
    
    pvm::execute(instructions).unwrap();
    //fs::write(output_file, output).unwrap();
}
