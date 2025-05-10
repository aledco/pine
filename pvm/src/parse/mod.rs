use crate::inst::Instruction;

mod lex;
pub(crate) mod token;

use lex::*;
use token::*;

use crate::inst::*;
use crate::operand::*;

pub fn parse(input: &str) -> Result<Vec<Box<dyn Instruction>>, String> {
    let lines = lex(input);
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

const OPERAND_FORMATS: [OperandFormat;3] = [OperandFormat::Variable, OperandFormat::Value, OperandFormat::Value];

impl Parse for AddInst {
    // TODO put this in inst macro
    fn parse(line: &Line) -> Result<Box<dyn Instruction>, String> {
        if line.inst_token != Token::Identifier(String::from(Self::NAME))
            || line.operand_tokens.len() != Self::N_OPERANDS
        {
            return Err(format!("Cannot parse instruction from line {}", line.line));
        }

        if line.operand_tokens.len() != Self::N_OPERANDS {
            return Err(format!(
                "Invalid number of operands for {}. Expected {} but got {}",
                Self::NAME,
                Self::N_OPERANDS,
                line.operand_tokens.len()
            ));
        }

        let mut operands: Vec<Operand> = Vec::new();
        for (operand_token, operand_format) in line.operand_tokens.iter().zip(OPERAND_FORMATS.iter()) {
            let operand = match operand_token {
                Token::Identifier(v) => {
                    match operand_format {
                        OperandFormat::Variable | OperandFormat::Value  => Ok(Operand::Variable(v.clone())),
                        OperandFormat::Label => Ok(Operand::Label(v.clone())),
                        _=> Err(format!("Error at line {}: invalid operand format", line.line)),
                    }
                }
                Token::Literal(l) => {
                    match operand_format {
                        OperandFormat::Constant | OperandFormat::Value => {
                            match l {
                                Literal::Integer(i) => Ok(Operand::Constant(i.clone() as u64)),
                                Literal::Float(f) => unimplemented!(),
                                Literal::Char(c) => Ok(Operand::Constant(c.clone() as u64)),
                                Literal::String(_) => unimplemented!(),
                            }
                        }
                        _ => Err(format!("Error at line {}: invalid operand format", line.line))
                    }
                },
            };

            match operand {
                Ok(operand) => operands.push(operand),
                Err(e) => return Err(e),
            }
        }

        // TODO parse operands, create instruction

        let o1 = operands.remove(0);
        let o2 = operands.remove(0);
        let o3 = operands.remove(0);
        Ok(Box::new(Self::new(o1, o2, o3)))
    }
}
