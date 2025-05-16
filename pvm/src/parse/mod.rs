mod lex;
pub(crate) mod token;
pub(crate) mod error;

use lex::*;
pub(crate) use token::*;
pub(crate) use error::*;
use crate::inst::*;
use crate::error::Error;

pub fn parse(input: &str) -> Result<Vec<Box<dyn Instruction>>, Error> {
    let lines = lex(input)?;
    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
    for line in &lines {
        let inst = parse_instruction(line)?;
        instructions.push(inst);
    }

    Ok(instructions)
}

fn parse_instruction(line: &Line) -> Result<Box<dyn Instruction>, Error> {
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
            LenInst::NAME => LenInst::parse(line),
            LoadInst::NAME => LoadInst::parse(line),
            LoadByteInst::NAME => LoadByteInst::parse(line),
            StoreInst::NAME => StoreInst::parse(line),
            StoreByteInst::NAME => StoreByteInst::parse(line),
            PrintiInst::NAME => PrintiInst::parse(line),
            PrintfInst::NAME => PrintfInst::parse(line),
            PrintcInst::NAME => PrintcInst::parse(line),
            PrintlnInst::NAME => PrintlnInst::parse(line),
            inst => Err(ParseError::inst_not_recognized(inst, line.line))
        },
        _ => Err(ParseError::invalid_token(line.line)),
    }
}

pub(crate) trait Parse {
    fn parse(line: &Line) -> Result<Box<dyn Instruction>, Error>;
}
