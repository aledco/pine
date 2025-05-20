pub mod lex;
pub mod parse;
mod ast;
mod operator;
mod token;
mod error;
mod symbol;

pub use ast::*;
pub use operator::*;
pub use symbol::*;
pub use token::*;
pub use error::*;

pub fn parse(input: String) -> ParseResult<Program> {
    let tokens = lex::lex(input)?;
    parse::parse(tokens)
}
