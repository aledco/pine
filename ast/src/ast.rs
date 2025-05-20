use crate::operator::Operator;
use crate::symbol::*;
use crate::token::*;
use std::fmt;

extern crate ast_proc_macros;
use ast_proc_macros::*;

/// The type of Pine construct.
#[derive(Clone, Debug, PartialEq)] // TODO move to different module
pub enum PineType {
    Integer,
    Float,
    Bool,
    String,
    List(Box<PineType>),
    Function {
        params: Vec<PineType>,
        ret: Box<PineType>,
    },
    Void,
    Unknown,
}

impl Default for PineType {
    fn default() -> Self {
        PineType::Unknown
    }
}

/// The `Ast` trait.
pub trait Ast: fmt::Debug {
    fn scope(&self) -> ScopeRef;
    fn set_scope(&mut self, scope: ScopeRef);
    fn span(&self) -> Span;
}

/// The `TypedAst` trait.
pub trait TypedAst: Ast + fmt::Debug {
    fn pine_type(&self) -> PineType;
    fn set_pine_type(&mut self, pine_type: PineType);
}

/// Represents a Pine program.
#[ast]
pub struct Program {
    pub functions: Vec<Function>,
    //pub main: AstNode,
}

/// Represents a Pine function.
#[ast]
pub struct Function {
    pub identifier: Box<Identifier>,
    pub params: Vec<Param>,
    pub return_type_node: Option<Box<TypeNode>>,
    pub body: Box<Statement>,
}

/// Represents a Pine parameter.
#[ast]
pub struct Param {
    pub identifier: Box<Identifier>,
    pub type_node: Box<TypeNode>,
}

/// Represents a Pine statement type.
#[derive(Debug)]
pub enum StatementType { // TODO make statement and enum and each statement type a struct
    Let(Box<Identifier>, Option<Box<TypeNode>>, Box<Expression>),
    Set(Box<Identifier>, Box<Expression>),
    If(Box<Expression>, Box<Statement>, Option<Box<Statement>>), // TODO add elifs
    While(Box<Expression>, Box<Statement>),
    Return(Option<Box<Expression>>),
    Expression(Box<Expression>),
    Block(Vec<Statement>),
}

/// Represents a Pine statement.
#[ast]
pub struct Statement {
    pub statement_type: StatementType,
}

#[derive(Debug)]
pub enum ExpressionType { // TODO make expression an enum and each expression type a struct
    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    StringLiteral(String),
    Identifier(Box<Identifier>),
    Unary(Operator, Box<Expression>),
    Binary(Box<Expression>, Operator, Box<Expression>),
}

#[typed_ast]
pub struct Expression {
    pub expression_type: ExpressionType,
}

#[ast]
pub struct Identifier {
    pub name: String,
    #[default(Symbol::default)]
    pub symbol: SymbolRef,
}

impl Identifier {
    pub fn pine_type(&mut self) -> PineType {
        self.symbol.borrow().pine_type.clone()
    }

    pub fn set_pine_type(&mut self, pine_type: PineType) {
        self.symbol.borrow_mut().pine_type = pine_type;
    }
}

#[derive(TypedAst)]
#[ast]
pub struct TypeNode {
    pine_type: PineType,
}
