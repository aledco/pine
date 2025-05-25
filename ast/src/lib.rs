pub mod lex;
pub mod parse;
mod ast;
mod operator;
mod token;
mod error;
mod symbol;
mod sem;

pub use ast::*;
pub use operator::*;
pub use symbol::*;
pub use token::*;
pub use error::*;

/// Parses a Pine input program into an AST. Returns the annotated AST.
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
pub fn parse<T>(input: T) -> Result<Program, Error>
where T: Into<String> {
    let main_module = parse_module(input)?;
    
    let mut program = Program::new(Box::new(main_module));
    
    // let modules = sem::modresv::resolve_modules(&main_module);

    // check function returns
    sem::ret::check(&mut program)?;

    // annotate the AST with scopes
    sem::scoping::global(&mut program)?;
    sem::scoping::local(&mut program)?;

    // annotate the AST with types
    sem::typing::global(&mut program)?;
    sem::typing::local(&mut program)?;
    
    Ok(program)
}

/// Parses a module.
pub(crate) fn parse_module<T>(input: T) -> Result<Module, Error>
where T: Into<String> {
    let tokens = lex::lex(input.into())?;
    parse::parse(tokens)
}
