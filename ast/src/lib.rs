pub mod ast;
pub mod lex;
pub mod operator;
pub mod parse;
pub mod symbol;
pub mod token;
pub mod traverse;

// TODO move tests into this crate so we can make mods private

// TODO use pub(crate) to make functions internally public
pub fn parse(input: String) -> ast::Program {
    let tokens = lex::lex(input);
    let mut program = parse::parse(tokens);
    traverse::traverse(&mut program);
    program
}
