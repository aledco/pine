pub mod ast;
pub mod lex;
pub mod operator;
mod parse;
pub mod symbol;
pub mod token;

pub use ast::*;
pub use operator::*;
pub use symbol::*;
pub use token::*;

pub fn parse(input: String) -> Program {
    let tokens = lex::lex(input);
    parse::parse(tokens)
}
