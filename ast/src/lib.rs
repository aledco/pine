mod parse;
mod lex;
mod ast;

pub use self::ast::*; // export the ast module

pub fn test() -> i32 {
    return parse::test();
}
