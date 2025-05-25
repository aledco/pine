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
use crate::sem::SemError;

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

    // TODO move this code
    let main_fun = match main_module.scope().borrow().lookup("main") {
        Some(main_symbol) => main_symbol,
        None => return Err(SemError::error("no main function found", main_module.span())),
    };

    match &main_fun.borrow().pine_type {
        PineType::Function { ret, .. } => { // TODO ensure params make sense too
            if **ret != PineType::Void && **ret != PineType::Integer {
                return Err(SemError::error("main must return void or int", main_module.span()))
            }
        },
        _ => return Err(SemError::error("main must be a function", main_module.span()))
    }

    // TODO move to new fn
    let program = ast::Program {
        main_module: Box::new(main_module),
        main_fun,
    };

    Ok(program)
}

// TODO comment
pub fn parse_module<T>(input: T) -> Result<Module, Error>
where T: Into<String> {
    let tokens = lex::lex(input.into())?;
    let mut module = parse::parse(tokens)?;

    // check function returns
    sem::ret::check(&mut module)?;

    // annotate the AST with scopes
    sem::scoping::global(&mut module)?;
    sem::scoping::local(&mut module)?;

    // annotate the AST with types
    sem::typing::global(&mut module)?;
    sem::typing::local(&mut module)?;

    Ok(module)
}
