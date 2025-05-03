use std::cell::RefCell;
use crate::token::*;
use crate::symbol::*;

/// The type of Pine construct.
#[derive(Clone, Debug, PartialEq)]
pub enum PineType {
    Integer,
    Float,
    String,
    List(Box<PineType>),
    Function {
        params: Vec<PineType>,
        ret: Box<PineType>,
    },
    Void,
    Unknown,
}

#[derive(Debug)]
pub enum AstType {
    Function {
        identifier: Box<AstNode>,
        params: Vec<AstNode>,
        body: Box<AstNode>,
    },
    Block(Vec<AstNode>),
    Let {
        identifier: Box<AstNode>,
        expression: Box<AstNode>,
    },
    BinaryExpression {
        lhs: Box<AstNode>,
        op: Operator,
        rhs: Box<AstNode>,
    },
    UnaryExpression {
        op: Operator,
        expr: Box<AstNode>,
    },
    IdentifierExpression(Box<AstNode>),
    IntegerExpression(i64),
    FloatExpression(f64),
    StringExpression(String),
    Identifier(SymbolRef),
    Dummy,
}

#[derive(Debug)]
pub struct AstNode {
    pub ast_type: AstType,
    pub pine_type: PineType,
    pub scope: ScopeRef,
    pub span: Span,
}

impl AstNode {
    pub fn new_function(
        identifier: Box<AstNode>,
        params: Vec<AstNode>,
        body: Box<AstNode>,
        pine_type: PineType,
        scope: ScopeRef,
        span: Span,
    ) -> Self {
        Self {
            ast_type: AstType::Function {
                identifier,
                params,
                body,
            },
            pine_type,
            scope,
            span,
        }
    }

    pub fn new_block(statements: Vec<AstNode>, scope: ScopeRef, span: Span) -> Self {
        Self {
            ast_type: AstType::Block(statements),
            pine_type: PineType::Void,
            scope,
            span,
        }
    }
    
    pub fn new_let(identifier: Box<AstNode>, expression: Box<AstNode>, scope: ScopeRef, span: Span) -> Self {
        Self {
            ast_type: AstType::Let {
                identifier,
                expression
            },
            pine_type: PineType::Void,
            scope,
            span,
        }
    }

    pub fn new_binary_expression(lhs: Box<AstNode>, op: Operator, rhs: Box<AstNode>, scope: ScopeRef, span: Span) -> Self {
        Self {
            ast_type: AstType::BinaryExpression {
                lhs,
                op,
                rhs
            },
            pine_type: PineType::Unknown,
            scope,
            span
        }
    }

    pub fn new_unary_expression(op: Operator, expr: Box<AstNode>, scope: ScopeRef, span: Span) -> Self {
        Self {
            ast_type: AstType::UnaryExpression {
                op,
                expr
            },
            pine_type: PineType::Unknown,
            scope,
            span
        }
    }
    
    pub fn new_identifier_expression(identifier: Box<AstNode>, scope: ScopeRef, span: Span) -> Self {
        Self {
            ast_type: AstType::IdentifierExpression(identifier),
            pine_type: PineType::Unknown,
            scope,
            span
        }
    }

    pub fn new_integer_expression(value: i64, scope: ScopeRef, span: Span) -> Self {
        Self {
            ast_type: AstType::IntegerExpression(value),
            pine_type: PineType::Unknown,
            scope,
            span
        }
    }

    pub fn new_float_expression(value: f64, scope: ScopeRef, span: Span) -> Self {
        Self {
            ast_type: AstType::FloatExpression(value),
            pine_type: PineType::Unknown,
            scope,
            span
        }
    }

    pub fn new_string_expression(value: String, scope: ScopeRef, span: Span) -> Self {
        Self {
            ast_type: AstType::StringExpression(value),
            pine_type: PineType::Unknown,
            scope,
            span
        }
    }
    
    pub fn new_identifier(value: SymbolRef, scope: ScopeRef, span: Span) -> Self {
        Self {
            ast_type: AstType::Identifier(value),
            pine_type: PineType::Unknown,
            scope,
            span,
        }
    }

    pub fn dummy() -> Self {
        Self {
            ast_type: AstType::Dummy,
            pine_type: PineType::Void,
            scope: Scope::new_global(),
            span: Span::from(Point::from(0, 0), Point::from(0, 0)),
        }
    }
}

pub struct Program {
    pub functions: Vec<AstNode>,
    //pub main: AstNode,
}

impl Program {
    pub fn new(functions: Vec<AstNode>) -> Self {
        Self {
            functions
        }
    }
}
