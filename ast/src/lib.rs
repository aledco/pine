mod token;
mod lex;
mod ast;
mod parse;

pub use self::token::*; // export the token module
pub use self::ast::*; // export the ast module

pub fn parse(input: String) -> Program {
    let tokens = lex::lex(input);
    parse::parse(tokens)
}
