pub mod ast;
pub mod lex;
pub mod parse;
pub mod symbol;
pub mod token;

// pub use self::token::*; // export the token module
// pub use self::symbol::*; // export the symbol module
// pub use self::ast::*; // export the ast module

pub fn parse(input: String) -> ast::Program {
    let tokens = lex::lex(input);
    parse::parse(tokens)
}
