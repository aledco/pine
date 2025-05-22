use crate::ast::*;
use crate::operator::Operator;
use crate::token::*;
use std::fmt::Debug;
use crate::error::{ParseError, ParseResult};

pub fn parse(tokens: Vec<Token>) -> ParseResult<Program> {
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

    pub fn parse(&mut self) -> ParseResult<Program> {
        let mut functions = vec![];
        while !self.eof() {
            if self.matches_function() {
                let function = self.parse_function()?;
                functions.push(function);
            } else {
                Err(ParseError::error("expected function", self.span()))?
            }
        }

        let span = if !functions.is_empty() {
            functions.first().unwrap().span() + functions.last().unwrap().span()
        } else {
            Span::default()
        };
        Ok(Program::new(functions, span))
    }

    fn parse_function(&mut self) -> ParseResult<Fun> {
        let fun = self.match_token(Keyword::Fun)?;
        let identifier = self.parse_identifier()?;
        let params = self.parse_params()?;

        // compute the type of the function
        let return_type = if self.matches(Punctuation::Arrow) {
            self.match_token(Punctuation::Arrow)?;
            Some(Box::new(self.parse_type()?))
        } else {
            None
        };

        self.match_token(Keyword::Begin)?;
        let body = self.parse_block()?;
        self.match_token(Keyword::End)?;

        let span = fun.span + body.span();
        Ok(Fun::new(
            Box::new(identifier),
            params,
            return_type,
            Box::new(body),
            span,
        ))
    }

    fn parse_params(&mut self) -> ParseResult<Vec<Param>> {
        self.match_token(Punctuation::OpenParen)?;
        let mut params = Vec::new();
        while !self.matches(Punctuation::CloseParen) {
            let param = self.parse_param()?;
            params.push(param);
            if !self.matches(Punctuation::Comma) {
                break;
            }
            
            self.match_token(Punctuation::Comma)?;
        }

        self.match_token(Punctuation::CloseParen)?;
        Ok(params)
    }

    fn parse_param(&mut self) -> ParseResult<Param> {
        let identifier = self.parse_identifier()?;
        self.match_token(Punctuation::Colon)?;
        let type_node = self.parse_type()?;
        let span = identifier.span() + type_node.span();
        Ok(Param::new(Box::new(identifier), Box::new(type_node), span))
    }

    fn parse_identifier(&mut self) -> ParseResult<Ident> {
        let token = self.match_token(TokenTypeMatch::Identifier)?;
        match token.token_type {
            TokenType::Identifier(identifier) => Ok(Ident::new(identifier, token.span)),
            _ => panic!("parser bug"),
        }
    }

    fn parse_block(&mut self) -> ParseResult<Block> {
        let mut span = self.span();
        let mut statements = vec![];
        while self.matches_statement() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        if let Some(statement) = statements.last() {
            span = span + statement.span();
        }

        Ok(Block::new(statements, span))
    }

    fn parse_statement(&mut self) -> ParseResult<Stmt> {
        if self.matches(Keyword::Let) {
            Ok(Stmt::Let(self.parse_let()?))
        } else if self.matches(Keyword::Set) {
            Ok(Stmt::Set(self.parse_set()?))
        } else if self.matches(Keyword::If) {
            Ok(Stmt::If(self.parse_if()?))
        } else if self.matches(Keyword::While) {
            Ok(Stmt::While(self.parse_while()?))
        } else if self.matches(Keyword::Return) {
            Ok(Stmt::Return(self.parse_return()?))
        } else if self.matches(Keyword::Begin) {
            self.match_token(Keyword::Begin)?;
            let block = self.parse_block()?;
            self.match_token(Keyword::End)?;
            Ok(Stmt::Block(block))
        } else if self.matches_expression() {
            let expr = self.parse_expression()?;
            let span = expr.span();
            Ok(Stmt::Expr(ExprStmt::new(Box::new(expr), span)))
        } else {
            Err(ParseError::error("invalid statement", self.span()))
        }
    }

    fn parse_let(&mut self) -> ParseResult<LetStmt> {
        let let_token = self.match_token(Keyword::Let)?;
        let identifier = self.parse_identifier()?;
        let type_node = if self.matches(Punctuation::Colon) {
            self.match_token(Punctuation::Colon)?;
            let type_node = self.parse_type()?;
            Some(Box::new(type_node))
        } else {
            None
        };

        self.match_token(Punctuation::EqualSign)?;
        let expression = self.parse_expression()?;
        let span = let_token.span + expression.span();
        Ok(LetStmt::new(
            Box::new(identifier), type_node, Box::new(expression),
            span,
        ))
    }

    fn parse_set(&mut self) -> ParseResult<SetStmt> {
        let let_token = self.match_token(Keyword::Set)?;
        let identifier = self.parse_identifier()?;
        self.match_token(Punctuation::EqualSign)?;
        let expression = self.parse_expression()?;
        let span = let_token.span + expression.span();
        Ok(SetStmt::new(
            Box::new(identifier), Box::new(expression),
            span,
        ))
    }

    fn parse_if(&mut self) -> ParseResult<IfStmt> {
        let if_token = self.match_token(Keyword::If)?;
        let cond = self.parse_expression()?;
        self.match_token(Keyword::Then)?;
        let if_body = self.parse_block()?;

        let mut conds = vec![cond];
        let mut blocks = vec![if_body];
        while self.matches(Keyword::Elif) {
            self.match_token(Keyword::Elif)?;
            let elif_cond = self.parse_expression()?;
            self.match_token(Keyword::Then)?;
            let elif_body = self.parse_block()?;
            conds.push(elif_cond);
            blocks.push(elif_body);
        }

        let else_body = if self.matches(Keyword::Else) {
            self.match_token(Keyword::Else)?;
            let else_body = self.parse_block()?;
            Some(Box::new(else_body))
        } else {
            None
        };
        let end = self.match_token(Keyword::End)?;
        let span = if_token.span + end.span;
        Ok(IfStmt::new(
            conds, blocks, else_body,
            span,
        ))
    }

    fn parse_while(&mut self) -> ParseResult<WhileStmt> {
        let while_token = self.match_token(Keyword::While)?;
        let condition = self.parse_expression()?;
        self.match_token(Keyword::Do)?;
        let body = self.parse_block()?;
        let end = self.match_token(Keyword::End)?;
        let span = while_token.span + end.span;
        Ok(WhileStmt::new(
            Box::new(condition), Box::new(body),
            span,
        ))
    }

    fn parse_return(&mut self) -> ParseResult<ReturnStmt> {
        let ret = self.match_token(Keyword::Return)?;
        let (expression, span) = if self.matches_expression() {
            let e = self.parse_expression()?;
            let s = ret.span + e.span();
            (Some(Box::new(e)), s)
        } else {
            (None, ret.span)
        };

        Ok(ReturnStmt::new(expression, span))
    }

    fn parse_expression(&mut self) -> ParseResult<Expr> {
        self.parse_expression_by_precedence(Operator::max_precedence())
    }

    fn parse_expression_by_precedence(&mut self, precedence: i32) -> ParseResult<Expr> {
        if precedence < Operator::min_precedence() {
            self.parse_expression_term()
        } else {
            let mut expr = self.parse_expression_by_precedence(precedence - 1)?;
            while self.matches_any(Operator::binary_ops_by_precedence(precedence)) {
                let op_token = self.match_any(Operator::binary_ops_by_precedence(precedence))?;
                let op = match op_token.token_type {
                    TokenType::Operator(op) => op,
                    _ => panic!("parser bug"),
                };
                let rhs = self.parse_expression_by_precedence(precedence - 1)?;
                let span = expr.span() + self.span();
                expr = Expr::Binary(BinaryExpr::new(
                    Box::new(expr), op, Box::new(rhs),
                    span,
                ))
            }

            Ok(expr)
        }
    }

    fn parse_expression_term(&mut self) -> ParseResult<Expr> {
        let mut expr = if self.matches(TokenTypeMatch::Identifier) {
            Ok(Expr::Ident(self.parse_identifier_expression()?))
        } else if self.matches(TokenTypeMatch::Integer) {
            Ok(Expr::IntLit(self.parse_integer()?))
        } else if self.matches(TokenTypeMatch::Float) {
            Ok(Expr::FloatLit(self.parse_float()?))
        } else if self.matches_any(vec![Keyword::True, Keyword::False]) {
            Ok(Expr::BoolLit(self.parse_bool()?))
        } else if self.matches(TokenTypeMatch::String) {
            Ok(Expr::StringLit(self.parse_string()?))
        } else if self.matches_any(Operator::all_unary_ops()) {
            Ok(Expr::Unary(self.parse_unary_expression()?))
        } else if self.matches(Punctuation::OpenParen) {
            self.parse_parenthized_expression()
        } else {
            Err(ParseError::error("invalid expression", self.span()))
        }?;

        // check for function call or indexing expression
        if self.matches(Punctuation::OpenParen) {
            let (args, span) = self.parse_function_call_args()?;
            expr = Expr::Call(CallExpr::new(Box::new(expr), args, span));
        } // TODO index expr

        Ok(expr)
    }

    fn parse_function_call_args(&mut self) -> ParseResult<(Vec<Expr>, Span)> {
        let mut args: Vec<Expr> = vec![];
        let open = self.match_token(Punctuation::OpenParen)?;
        while !self.matches(Punctuation::CloseParen) {
            let arg = self.parse_expression()?;
            args.push(arg);
            if !self.matches(Punctuation::Comma) {
                break;
            }
            
            self.match_token(Punctuation::Comma)?;
        }

        let close = self.match_token(Punctuation::CloseParen)?;
        let span = open.span + close.span;
        Ok((args, span))
    }

    fn parse_unary_expression(&mut self) -> ParseResult<UnaryExpr> {
        let op_token = self.match_any(Operator::all_unary_ops())?;
        let op = match op_token.token_type {
            TokenType::Operator(op) => op,
            _ => panic!("parser bug"),
        };
        let expr = self.parse_expression_term()?;
        let span = op_token.span + expr.span();
        Ok(UnaryExpr::new(op, Box::new(expr), span))
    }

    fn parse_parenthized_expression(&mut self) -> ParseResult<Expr> {
        self.match_token(Punctuation::OpenParen)?;
        let expr = self.parse_expression();
        self.match_token(Punctuation::CloseParen)?;
        expr
    }

    fn parse_identifier_expression(&mut self) -> ParseResult<IdentExpr> {
        let identifier = self.parse_identifier()?;
        let span = identifier.span();
        Ok(IdentExpr::new(Box::new(identifier), span))
    }

    fn parse_integer(&mut self) -> ParseResult<IntLitExpr> {
        let token = self.match_token(TokenTypeMatch::Integer)?;
        match token.token_type {
            TokenType::Integer(value) => {
                Ok(IntLitExpr::new(value, token.span.clone()))
            }
            _ => panic!("parser bug"),
        }
    }

    fn parse_float(&mut self) -> ParseResult<FloatLitExpr> {
        let token = self.match_token(TokenTypeMatch::Float)?;
        match token.token_type {
            TokenType::Float(value) => {
                Ok(FloatLitExpr::new(value, token.span.clone()))
            }
            _ => panic!("parser bug"),
        }
    }

    fn parse_bool(&mut self) -> ParseResult<BoolLitExpr> {
        let token = self.match_any(vec![Keyword::True, Keyword::False])?;
        match token.token_type {
            TokenType::Keyword(Keyword::False) => {
                Ok(BoolLitExpr::new(false, token.span.clone()))
            }
            TokenType::Keyword(Keyword::True) => {
                Ok(BoolLitExpr::new(true, token.span.clone()))
            }
            _ => panic!("parser bug"),
        }
    }

    fn parse_string(&mut self) -> ParseResult<StringLitExpr> {
        let token = self.match_token(TokenTypeMatch::String)?;
        match token.token_type {
            TokenType::String(value) => {
                Ok(StringLitExpr::new(value, token.span.clone()))
            }
            _ => panic!("parser bug"),
        }
    }

    fn parse_type(&mut self) -> ParseResult<Ty> {
        let span = self.span();
        let pine_type = self.match_type()?;
        Ok(Ty::new(pine_type, span))
    }

    fn match_type(&mut self) -> ParseResult<PineType> {
        if self.matches(Keyword::Void) {
            self.match_token(Keyword::Void)?;
            Ok(PineType::Void)
        } else if self.matches(Keyword::Int) {
            self.match_token(Keyword::Int)?;
            Ok(PineType::Integer)
        } else if self.matches(Keyword::Float) {
            self.match_token(Keyword::Float)?;
            Ok(PineType::Float)
        } else if self.matches(Keyword::Bool) {
            self.match_token(Keyword::Bool)?;
            Ok(PineType::Bool)
        } else if self.matches(Keyword::String) {
            self.match_token(Keyword::String)?;
            Ok(PineType::String)
        } else if self.matches(Punctuation::OpenBracket) {
            self.match_token(Punctuation::OpenBracket)?;
            let elem_type = self.match_type()?;
            self.match_token(Punctuation::OpenBracket)?;
            Ok(PineType::List(Box::new(elem_type)))
        } else {
            // TODO parse function and user defined types
            Err(ParseError::error("invalid type", self.span()))
        }
    }

    fn match_token<T>(&mut self, token_type: T) -> ParseResult<Token>
    where
        T: TokenMatch + Copy + Debug,
    {
        let token = self.token();
        if token_type.matches(&token.token_type) {
            self.index += 1;
            Ok(token)
        } else {
            Err(ParseError::error(format!("expected {:?} found {:?}", token_type, token.token_type), self.span()))
        }
    }

    fn match_any<T>(&mut self, token_types: Vec<T>) -> ParseResult<Token>
    where
        T: TokenMatch + Copy + Debug,
    {
        for token_type in &token_types {
            if self.matches(*token_type) {
                return self.match_token(*token_type);
            }
        }

        Err(ParseError::error(format!("expected one of {:?} found {:?}", token_types, self.token_type()), self.span()))
    }

    fn matches<T>(&self, token_type: T) -> bool
    where
        T: TokenMatch + Copy + Debug,
    {
        token_type.matches(&self.token_type())
    }

    fn matches_any<T>(&self, token_types: Vec<T>) -> bool
    where
        T: TokenMatch + Copy + Debug,
    {
        token_types.into_iter().any(|t| self.matches(t))
    }

    fn matches_function(&self) -> bool {
        self.matches(Keyword::Fun)
    }

    fn matches_statement(&self) -> bool {
        if self.matches_any(vec![
            Keyword::Begin,
            Keyword::Let,
            Keyword::Set,
            Keyword::If,
            Keyword::For,
            Keyword::While,
            Keyword::Return,
        ]) {
            true
        } else if self.matches_expression() {
            true
        } else {
            false
        }
    }

    fn matches_expression(&self) -> bool {
        if self.matches_any(vec![
            TokenTypeMatch::Identifier,
            TokenTypeMatch::Integer,
            TokenTypeMatch::Float,
            TokenTypeMatch::String,
        ]) {
            true
        } else if self.matches_any(vec![Keyword::True, Keyword::False]) {
            true
        } else if self.matches_any(Operator::all_unary_ops().into_iter().map(|o| o).collect()) {
            true
        } else if self.matches_any(vec![Punctuation::OpenParen, Punctuation::OpenBracket]) {
            true
        } else {
            false
        }
    }

    fn token(&self) -> Token {
        self.tokens[self.index].clone()
    }

    fn token_type(&self) -> TokenType {
        self.tokens[self.index].token_type.clone()
    }

    fn span(&self) -> Span {
        self.tokens[self.index].span.clone()
    }

    fn eof(&self) -> bool {
        self.index >= self.tokens.len()
    }
}
