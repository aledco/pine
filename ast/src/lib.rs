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

/// Parses a Pine input program into an AST.
/// 
/// # Arguments
/// - `input` - the Pine input.
/// 
/// # Returns
/// The parse result containing the program AST if successful.
/// 
/// # Examples
/// ```
/// let input = "fun main() begin end";
/// let program = ast::parse(input).unwrap();
/// ```
pub fn parse<T>(input: T) -> ParseResult<Program>
where T: Into<String> {
    let tokens = lex::lex(input.into())?;
    parse::parse(tokens)
}
