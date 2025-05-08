pub mod ast;
pub mod lex;
pub mod operator;
pub mod symbol;
pub mod token;
mod parse;
mod traverse;

pub use token::*;
pub use operator::*;
pub use symbol::*;
pub use ast::*;

pub fn parse(input: String) -> Program {
    let tokens = lex::lex(input);
    let mut program = parse::parse(tokens);
    traverse::traverse(&mut program);
    program
}
