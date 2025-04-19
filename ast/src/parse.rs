use crate::token::*;
use crate::ast::*;

pub fn parse(tokens: Vec<Token>) -> Program {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

/// Represents the Pine parser
struct Parser {
    /// The vector of tokens representing the Pine program
    tokens: Vec<Token>,
    /// The index into the tokens
    index: usize,
    
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, index: 0 }
    }
    
    pub fn parse(&mut self) -> Program {
        Program{ nodes: vec![], main: FunctionDef {} }
    }
}
