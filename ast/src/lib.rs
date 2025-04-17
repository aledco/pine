mod parse;
mod lex;
mod ast;
mod token;

pub use self::ast::*; // export the ast module

pub fn parse(input: String) -> Program {
    let tokens = lex::lex(input);
    parse::parse(tokens)
}

