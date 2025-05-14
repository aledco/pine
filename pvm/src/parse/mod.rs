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
            AddfInst::NAME => AddfInst::parse(line),
            SubInst::NAME => SubInst::parse(line),
            SubfInst::NAME => SubfInst::parse(line),
            MulInst::NAME => MulInst::parse(line),
            MulfInst::NAME => MulfInst::parse(line),
            DivInst::NAME => DivInst::parse(line),
            DivfInst::NAME => DivfInst::parse(line),
            ModInst::NAME => ModInst::parse(line),
            ModfInst::NAME => ModfInst::parse(line),
            PowInst::NAME => PowInst::parse(line),
            PowfInst::NAME => PowfInst::parse(line),
            EqInst::NAME => EqInst::parse(line),
            NeqInst::NAME => NeqInst::parse(line),
            LtInst::NAME => LtInst::parse(line),
            LteInst::NAME => LteInst::parse(line),
            GtInst::NAME => GtInst::parse(line),
            GteInst::NAME => GteInst::parse(line),
            MoveInst::NAME => MoveInst::parse(line),
            JumpInst::NAME => JumpInst::parse(line),
            JumpZeroInst::NAME => JumpZeroInst::parse(line),
            JumpNotZeroInst::NAME => JumpNotZeroInst::parse(line),
            LabelInst::NAME => LabelInst::parse(line),
            AllocInst::NAME => AllocInst::parse(line),
            DeallocInst::NAME => DeallocInst::parse(line),
            PrintiInst::NAME => PrintiInst::parse(line),
            PrintfInst::NAME => PrintfInst::parse(line),
            PrintcInst::NAME => PrintcInst::parse(line),
            PrintlnInst::NAME => PrintlnInst::parse(line),
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
