mod parse;
mod lex;
mod ast;
mod token;

pub use self::ast::*; // export the ast module

pub fn parse(input: String) -> Vec<Box<dyn AST>> {
    // let tokens = lex::lex(input)
    // return parse::parse(tokens)
    return vec![];
}
