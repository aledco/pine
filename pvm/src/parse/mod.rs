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
            AdduInst::NAME => AdduInst::parse(line),
            AddfInst::NAME => AddfInst::parse(line),
            SubInst::NAME => SubInst::parse(line),
            SubuInst::NAME => SubuInst::parse(line),
            SubfInst::NAME => SubfInst::parse(line),
            MulInst::NAME => MulInst::parse(line),
            MuluInst::NAME => MuluInst::parse(line),
            MulfInst::NAME => MulfInst::parse(line),
            DivInst::NAME => DivInst::parse(line),
            DivuInst::NAME => DivuInst::parse(line),
            DivfInst::NAME => DivfInst::parse(line),
            ModInst::NAME => ModInst::parse(line),
            ModuInst::NAME => ModuInst::parse(line),
            ModfInst::NAME => ModfInst::parse(line),
            PowInst::NAME => PowInst::parse(line),
            PowuInst::NAME => PowuInst::parse(line),
            PowfInst::NAME => PowfInst::parse(line),
            EqInst::NAME => EqInst::parse(line),
            EquInst::NAME => EquInst::parse(line),
            EqfInst::NAME => EqfInst::parse(line),
            NeqInst::NAME => NeqInst::parse(line),
            NeqfInst::NAME => NeqfInst::parse(line),
            NequInst::NAME => NequInst::parse(line),
            LtInst::NAME => LtInst::parse(line),
            LtuInst::NAME => LtuInst::parse(line),
            LtfInst::NAME => LtfInst::parse(line),
            LteInst::NAME => LteInst::parse(line),
            LteuInst::NAME => LteuInst::parse(line),
            LtefInst::NAME => LtefInst::parse(line),
            GtInst::NAME => GtInst::parse(line),
            GtuInst::NAME => GtuInst::parse(line),
            GtfInst::NAME => GtfInst::parse(line),
            GteInst::NAME => GteInst::parse(line),
            GteuInst::NAME => GteuInst::parse(line),
            GtefInst::NAME => GtefInst::parse(line),
            AndInst::NAME => AndInst::parse(line),
            OrInst::NAME => OrInst::parse(line),
            XorInst::NAME => XorInst::parse(line),
            ShlInst::NAME => ShlInst::parse(line),
            ShrInst::NAME => ShrInst::parse(line),
            ShraInst::NAME => ShraInst::parse(line),
            MoveInst::NAME => MoveInst::parse(line),
            NegInst::NAME => NegInst::parse(line),
            NegfInst::NAME => NegfInst::parse(line),
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
            FunInst::NAME => FunInst::parse(line),
            PushaInst::NAME => PushaInst::parse(line),
            PopaInst::NAME => PopaInst::parse(line),
            PushrInst::NAME => PushrInst::parse(line),
            PoprInst::NAME => PoprInst::parse(line),
            CallInst::NAME => CallInst::parse(line),
            RetInst::NAME => RetInst::parse(line),
            SaveInst::NAME => SaveInst::parse(line),
            RestoreInst::NAME => RestoreInst::parse(line),
            PrintiInst::NAME => PrintiInst::parse(line),
            PrintuInst::NAME => PrintuInst::parse(line),
            PrintfInst::NAME => PrintfInst::parse(line),
            PrinthInst::NAME => PrinthInst::parse(line),
            PrintbInst::NAME => PrintbInst::parse(line),
            PrintcInst::NAME => PrintcInst::parse(line),
            PrintsInst::NAME => PrintsInst::parse(line),
            PrintlnInst::NAME => PrintlnInst::parse(line),
            inst => Err(ParseError::inst_not_recognized(inst, line.line))
        },
        _ => Err(ParseError::invalid_token(line.line)),
    }
}

pub(crate) trait Parse {
    fn parse(line: &Line) -> Result<Box<dyn Instruction>, Error>;
}
