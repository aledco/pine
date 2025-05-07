pub mod ast;
pub mod lex;
pub mod parse;
pub mod symbol;
pub mod token;
//pub mod traverse;

pub fn parse(input: String) -> ast::Program {
    let tokens = lex::lex(input);
    let mut program = parse::parse(tokens);
    //traverse::traverse(&mut program);
    program
}
