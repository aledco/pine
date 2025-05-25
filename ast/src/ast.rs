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
    fn span(&self) -> Span;
}

/// The `ScopedAst` trait.
pub(crate) trait ScopedAst {
    fn scope(&self) -> ScopeRef;
    fn set_scope(&mut self, scope: ScopeRef);
}

/// Represents a Pine executable program.
pub struct Program {
    pub main_module: Box<Module>,
    pub main_fun: SymbolRef
}

/// Represents a Pine module.
#[ast]
pub struct Module {
    pub funs: Vec<Fun>, // TODO make top level enum
}

/// Represents a Pine function.
#[ast]
pub struct Fun {
    pub ident: Box<Ident>,
    pub params: Vec<Param>,
    pub return_ty: Option<Box<Ty>>,
    pub block: Box<Block>,
}

/// Represents a Pine parameter.
#[ast]
pub struct Param {
    pub ident: Box<Ident>,
    pub ty: Box<Ty>,
}

#[ast]
pub struct Import {
    pub ident: Box<Ident>,
    pub module: Box<Module>
}

/// Represents a Pine let statement.
#[ast]
pub struct LetStmt {
    pub ident: Box<Ident>,
    pub ty: Option<Box<Ty>>,
    pub expr: Box<Expr>,
}

/// Represents a Pine set statement.
#[ast]
pub struct SetStmt {
    pub ident: Box<Ident>,
    pub expr: Box<Expr>,
}

/// Represents a Pine if statement.
#[ast]
pub struct IfStmt {
    pub conds: Vec<Expr>,
    pub then_blocks: Vec<Block>,
    pub else_block: Option<Box<Block>>,
}

/// Represents a Pine while statement.
#[ast]
pub struct WhileStmt {
    pub cond: Box<Expr>,
    pub block: Box<Block>,
}

/// Represents a Pine return statement.
#[ast]
pub struct ReturnStmt {
    pub expr: Option<Box<Expr>>,
}

/// Represents a Pine expression statement.
#[ast]
pub struct ExprStmt {
    pub expr: Box<Expr>,
}

/// Represents a Pine block.
#[ast]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

/// Represents a Pine statement.
#[derive(Debug)]
pub enum Stmt {
    Let(LetStmt),
    Set(SetStmt),
    If(IfStmt),
    While(WhileStmt),
    Return(ReturnStmt),
    Expr(ExprStmt),
    Block(Block),
}

impl Ast for Stmt {
    fn span(&self) -> Span {
        match self {
            Stmt::Let(let_stmt) => let_stmt.span(),
            Stmt::Set(set_stmt) => set_stmt.span(),
            Stmt::If(if_stmt) => if_stmt.span(),
            Stmt::While(while_stmt) => while_stmt.span(),
            Stmt::Return(return_stmt) => return_stmt.span(),
            Stmt::Expr(expr_stmt) => expr_stmt.span(),
            Stmt::Block(block) => block.span(),
        }
    }
}

impl ScopedAst for Stmt {
    fn scope(&self) -> ScopeRef {
        match self {
            Stmt::Let(let_stmt) => let_stmt.scope(),
            Stmt::Set(set_stmt) => set_stmt.scope(),
            Stmt::If(if_stmt) => if_stmt.scope(),
            Stmt::While(while_stmt) => while_stmt.scope(),
            Stmt::Return(return_stmt) => return_stmt.scope(),
            Stmt::Expr(expr_stmt) => expr_stmt.scope(),
            Stmt::Block(block) => block.scope(),
        }
    }

    fn set_scope(&mut self, scope: ScopeRef) {
        match self {
            Stmt::Let(let_stmt) => let_stmt.set_scope(scope),
            Stmt::Set(set_stmt) => set_stmt.set_scope(scope),
            Stmt::If(if_stmt) => if_stmt.set_scope(scope),
            Stmt::While(while_stmt) => while_stmt.set_scope(scope),
            Stmt::Return(return_stmt) => return_stmt.set_scope(scope),
            Stmt::Expr(expr_stmt) => expr_stmt.set_scope(scope),
            Stmt::Block(block) => block.set_scope(scope),
        }
    }
}

/// Represents an integer literal.
#[ast]
pub struct IntLitExpr {
    pub value: i64,
    #[default(PineType::default)] pub ty: PineType,
    #[default(pvm::Operand::default)] pub dest: pvm::Operand,
}

/// Represents a floating point literal.
#[ast]
pub struct FloatLitExpr {
    pub value: f64,
    #[default(PineType::default)] pub ty: PineType,
    #[default(pvm::Operand::default)] pub dest: pvm::Operand,
}

/// Represents a bool literal.
#[ast]
pub struct BoolLitExpr {
    pub value: bool,
    #[default(PineType::default)] pub ty: PineType,
    #[default(pvm::Operand::default)] pub dest: pvm::Operand,
}

/// Represents a string literal.
#[ast]
pub struct StringLitExpr {
    pub value: String,
    #[default(PineType::default)] pub ty: PineType,
    #[default(pvm::Operand::default)] pub dest: pvm::Operand,
}

/// Represents an identifier expression.
#[ast]
pub struct IdentExpr {
    pub ident: Box<Ident>,
    #[default(PineType::default)] pub ty: PineType,
    #[default(pvm::Operand::default)] pub dest: pvm::Operand,
}

/// Represents a call expression.
#[ast]
pub struct CallExpr {
    pub fun: Box<Expr>,
    pub args: Vec<Expr>,
    #[default(PineType::default)] pub ty: PineType,
    #[default(pvm::Operand::default)] pub dest: pvm::Operand,
}

/// Represents a unary expression.
#[ast]
pub struct UnaryExpr {
    pub op: Operator,
    pub expr: Box<Expr>,
    #[default(PineType::default)] pub ty: PineType,
    #[default(pvm::Operand::default)] pub dest: pvm::Operand,
}

/// Represents a binary expression.
#[ast]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: Operator,
    pub right: Box<Expr>,
    #[default(PineType::default)] pub ty: PineType,
    #[default(pvm::Operand::default)] pub dest: pvm::Operand,
}

/// Represents a Pine expression.
#[derive(Debug)]
pub enum Expr {
    IntLit(IntLitExpr),
    FloatLit(FloatLitExpr),
    BoolLit(BoolLitExpr),
    StringLit(StringLitExpr),
    Ident(IdentExpr),
    Call(CallExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

impl Expr {
    pub fn ty(&self) -> PineType {
        match self {
            Expr::IntLit(int_lit) => int_lit.ty.clone(),
            Expr::FloatLit(float_lit) => float_lit.ty.clone(),
            Expr::BoolLit(bool_lit) => bool_lit.ty.clone(),
            Expr::StringLit(string_lit) => string_lit.ty.clone(),
            Expr::Ident(ident) => ident.ty.clone(),
            Expr::Call(call) => call.ty.clone(),
            Expr::Unary(unary) => unary.ty.clone(),
            Expr::Binary(binary) => binary.ty.clone(),
        }
    }

    pub fn set_ty(&mut self, ty: PineType) {
        match self {
            Expr::IntLit(int_lit) => int_lit.ty = ty,
            Expr::FloatLit(float_lit) => float_lit.ty = ty,
            Expr::BoolLit(bool_lit) => bool_lit.ty = ty,
            Expr::StringLit(string_lit) => string_lit.ty = ty,
            Expr::Ident(ident) => ident.ty = ty,
            Expr::Call(call) => call.ty = ty,
            Expr::Unary(unary) => unary.ty = ty,
            Expr::Binary(binary) => binary.ty = ty,
        }
    }

    pub fn dest(&self) -> pvm::Operand {
        match self {
            Expr::IntLit(int_lit) => int_lit.dest.clone(),
            Expr::FloatLit(float_lit) => float_lit.dest.clone(),
            Expr::BoolLit(bool_lit) => bool_lit.dest.clone(),
            Expr::StringLit(string_lit) => string_lit.dest.clone(),
            Expr::Ident(ident) => ident.dest.clone(),
            Expr::Call(call) => call.dest.clone(),
            Expr::Unary(unary) => unary.dest.clone(),
            Expr::Binary(binary) => binary.dest.clone(),
        }
    }
}
impl Ast for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::IntLit(int_lit_expr) => int_lit_expr.span(),
            Expr::FloatLit(float_lit_expr) => float_lit_expr.span(),
            Expr::BoolLit(bool_lit_expr) => bool_lit_expr.span(),
            Expr::StringLit(string_lit_expr) => string_lit_expr.span(),
            Expr::Ident(ident_expr) => ident_expr.span(),
            Expr::Call(call_expr) => call_expr.span(),
            Expr::Unary(unary_expr) => unary_expr.span(),
            Expr::Binary(binary_expr) => binary_expr.span(),
        }
    }
}

impl ScopedAst for Expr {
    fn scope(&self) -> ScopeRef {
        match self {
            Expr::IntLit(int_lit_expr) => int_lit_expr.scope(),
            Expr::FloatLit(float_lit_expr) => float_lit_expr.scope(),
            Expr::BoolLit(bool_lit_expr) => bool_lit_expr.scope(),
            Expr::StringLit(string_lit_expr) => string_lit_expr.scope(),
            Expr::Ident(ident_expr) => ident_expr.scope(),
            Expr::Call(call_expr) => call_expr.scope(),
            Expr::Unary(unary_expr) => unary_expr.scope(),
            Expr::Binary(binary_expr) => binary_expr.scope(),
        }
    }

    fn set_scope(&mut self, scope: ScopeRef) {
        match self {
            Expr::IntLit(int_lit_expr) => int_lit_expr.set_scope(scope),
            Expr::FloatLit(float_lit_expr) => float_lit_expr.set_scope(scope),
            Expr::BoolLit(bool_lit_expr) => bool_lit_expr.set_scope(scope),
            Expr::StringLit(string_lit_expr) => string_lit_expr.set_scope(scope),
            Expr::Ident(ident_expr) => ident_expr.set_scope(scope),
            Expr::Call(call_expr) => call_expr.set_scope(scope),
            Expr::Unary(unary_expr) => unary_expr.set_scope(scope),
            Expr::Binary(binary_expr) => binary_expr.set_scope(scope),
        }
    }
}

/// Represents a Pine identifier.
#[ast]
pub struct Ident {
    pub name: String,
    #[default(Symbol::default)]
    pub symbol: SymbolRef,
}

impl Ident {
    pub fn dest(&self) -> pvm::Operand {
        self.symbol.borrow().dest.clone()
    }
}

/// Represents a Pine type.
#[ast]
pub struct Ty {
    pub ty: PineType,
}
