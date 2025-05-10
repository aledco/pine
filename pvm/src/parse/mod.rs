use crate::inst::Instruction;

mod lex;
pub(crate) mod token;

use lex::*;
pub(crate) use token::*;

use crate::inst::*;

pub fn parse(input: &str) -> Result<Vec<Box<dyn Instruction>>, String> {
    let lines = lex(input)?;
    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
    for line in &lines {
        let inst = parse_instruction(line)?;
        instructions.push(inst);
    }

    Ok(instructions)
}

fn parse_instruction(line: &Line) -> Result<Box<dyn Instruction>, String> {
    match &line.inst_token {
        Token::Identifier(inst) => match inst.as_str() {
            AddInst::NAME => AddInst::parse(line),
            SubInst::NAME => SubInst::parse(line),
            MulInst::NAME => MulInst::parse(line),
            DivInst::NAME => DivInst::parse(line),
            ModInst::NAME => ModInst::parse(line),
            PowInst::NAME => PowInst::parse(line),
            code => Err(format!(
                "Error on line {}: instruction code {} not recognized",
                line.line, code
            )),
        },
        _ => Err(format!(
            "Error on line {}: line must begin with a valid instruction code",
            line.line
        )),
    }
}

pub(crate) trait Parse {
    fn parse(line: &Line) -> Result<Box<dyn Instruction>, String>;
}
