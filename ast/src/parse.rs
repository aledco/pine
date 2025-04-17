use crate::token::*;
use crate::ast::*;

pub fn parse(tokens: Vec<Token>) -> Program {
    Program{ modules: vec![], main: FunctionDef {} }
}