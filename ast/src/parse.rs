use std::fmt::Debug;
use crate::ast::*;
use crate::symbol::*;
use crate::token::*;

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
        let global_scope = Scope::new_global();
        let mut functions: Vec<AstNode> = vec![];
        while !self.eof() {
            if self.matches_function() {
                let function = self.parse_function(global_scope.clone());
                functions.push(function);
            } else {
                panic!("Parse Error: at {}", self.span())
            }
        }

        Program::new(functions)
    }

    fn parse_function(&mut self, scope: ScopeRef) -> AstNode {
        let param_scope = Scope::new_local(scope.clone());
        let body_scope = Scope::new_local(scope.clone());

        let fun = self.match_token(Keyword::Fun);
        let mut identifier = self.parse_identifier(scope.clone(), false);
        let params = self.parse_params(param_scope);

        // compute the type of the function
        let return_type = if self.matches(Punctuation::Arrow) {
            self.match_token(Punctuation::Arrow);
            self.parse_type()
        } else {
            PineType::Void
        };
        let param_types = params
            .iter()
            .map(|p| p.pine_type.clone())
            .collect::<Vec<PineType>>();
        let function_type = PineType::Function {
            params: param_types,
            ret: Box::new(return_type),
        };

        identifier.pine_type = function_type.clone();

        self.match_token(Keyword::Begin);
        let body = self.parse_block(body_scope);
        self.match_token(Keyword::End);

        let span = fun.span + body.span;
        AstNode::new_function(
            Box::new(identifier),
            params,
            Box::new(body),
            function_type,
            scope.clone(),
            span,
        )
    }

    fn parse_params(&mut self, scope: ScopeRef) -> Vec<AstNode> {
        self.match_token(Punctuation::OpenParen);
        let mut params = Vec::<AstNode>::new();
        while self.matches(TokenTypeMatch::Identifier) {
            let param = self.parse_param(scope.clone());
            params.push(param);
            if !self.matches(Punctuation::CloseParen) {
                break;
            }
        }

        self.match_token(Punctuation::CloseParen);
        params
    }

    fn parse_param(&mut self, scope: ScopeRef) -> AstNode {
        let mut identifier = self.parse_identifier(scope, false);
        self.match_token(Punctuation::Colon);
        identifier.pine_type = self.parse_type();
        identifier
    }

    fn parse_identifier(&mut self, scope: ScopeRef, lookup: bool) -> AstNode {
        let token = self.match_token(TokenTypeMatch::Identifier);
        match token.token_type {
            // TODO is this how it should work? I don't think so, need to rethink
            TokenType::Identifier(identifier) => {
                let symbol = scope.borrow().lookup(&identifier);
                if lookup {
                    match symbol {
                        Some(symbol) => AstNode::new_identifier(symbol, scope, token.span.clone()),
                        None => panic!("Name Error: at {}", self.span()), // TODO more descriptive names
                    }
                } else {
                    match symbol {
                        Some(_) => panic!("Name Error: at {}", self.span()), // TODO more descriptive names
                        None => {
                            let symbol = Symbol::new(identifier.clone());
                            scope.borrow_mut().add(symbol.clone());
                            AstNode::new_identifier(symbol, scope, token.span.clone())
                        }
                    }
                }
            }
            _ => panic!("Parse Error: at {}", self.span()),
        }
    }

    fn parse_block(&mut self, scope: ScopeRef) -> AstNode {
        let mut span = self.span();
        let mut statements: Vec<AstNode> = vec![];
        while self.matches_statement() {
            let statement = self.parse_statement(scope.clone());
            statements.push(statement);
        }

        if !statements.is_empty() {
            span = span + statements.last().unwrap().span.clone();
        }

        AstNode::new_block(statements, scope, span)
    }

    fn parse_statement(&mut self, scope: ScopeRef) -> AstNode {
        if self.matches(Keyword::Let) {
            self.parse_let(scope)
        } else if self.matches(Keyword::Set) {
            self.parse_set(scope)
        } else if self.matches(Keyword::If) {
            self.parse_if(scope)
        } else if self.matches(Keyword::While) {
            self.parse_while(scope)
        } else if self.matches(Keyword::For) {
            self.parse_for(scope)
        } else if self.matches(Keyword::Return) {
            self.parse_return(scope)
        } else if self.matches(Keyword::Begin) {
            self.match_token(Keyword::Begin);
            let block = self.parse_block(scope);
            self.match_token(Keyword::End);
            block
        } else if self.matches_expression() {
            self.parse_expression(scope)
        } else {
            panic!("Parse Error: at {}", self.span())
        }
    }

    fn parse_let(&mut self, scope: ScopeRef) -> AstNode {
        let let_token = self.match_token(Keyword::Let);
        let identifier = self.parse_identifier(scope.clone(), false);
        self.match_token(Punctuation::EqualSign);
        let expression = self.parse_expression(scope.clone());
        let span = let_token.span + expression.span;
        AstNode::new_let_statement(Box::new(identifier), Box::new(expression), scope, span)
    }

    fn parse_set(&mut self, scope: ScopeRef) -> AstNode {
        let let_token = self.match_token(Keyword::Set);
        let identifier = self.parse_identifier(scope.clone(), true);
        self.match_token(Punctuation::EqualSign);
        let expression = self.parse_expression(scope.clone());
        let span = let_token.span + expression.span;
        AstNode::new_set_statement(Box::new(identifier), Box::new(expression), scope, span)
    }

    fn parse_if(&mut self, scope: ScopeRef) -> AstNode {
        let if_token = self.match_token(Keyword::If);
        let condition = self.parse_expression(scope.clone());
        self.match_token(Keyword::Then);
        let if_body = self.parse_block(scope.clone());
        let else_body = if self.matches(Keyword::Else) {
            self.match_token(Keyword::Else);
            let else_body = self.parse_block(scope.clone());
            Some(Box::new(else_body))
        } else {
            None
        };
        let end = self.match_token(Keyword::End);
        let span = if_token.span + end.span;
        AstNode::new_if_statement(
            Box::new(condition),
            Box::new(if_body),
            else_body,
            scope,
            span,
        )
    }

    fn parse_while(&mut self, scope: ScopeRef) -> AstNode {
        let while_token = self.match_token(Keyword::While);
        let condition = self.parse_expression(scope.clone());
        self.match_token(Keyword::Do);
        let body = self.parse_block(scope.clone());
        let end = self.match_token(Keyword::End);
        let span = while_token.span + end.span;
        AstNode::new_while_statement(
            Box::new(condition),
            Box::new(body),
            scope,
            span,
        )
    }

    fn parse_for(&mut self, _scope: ScopeRef) -> AstNode {
        AstNode::dummy()
    }

    fn parse_return(&mut self, scope: ScopeRef) -> AstNode {
        let ret = self.match_token(Keyword::Return);
        let (expression, span) = if self.matches_expression() {
            let e = self.parse_expression(scope.clone());
            let s = ret.span + e.span.clone();
            (Some(Box::new(e)), s)
        } else {
            (None, ret.span)
        };

        AstNode::new_return_statement(expression, scope, span)
    }

    fn parse_expression(&mut self, scope: ScopeRef) -> AstNode {
        self.parse_expression_by_precedence(scope, Operator::max_precedence())
    }

    fn parse_expression_by_precedence(&mut self, scope: ScopeRef, precedence: i32) -> AstNode {
        if precedence < Operator::min_precedence() {
            self.parse_expression_term(scope)
        } else {
            let mut expr = self.parse_expression_by_precedence(scope.clone(), precedence - 1);
            while self.matches_any(Operator::binary_ops_by_precedence(precedence)) {
                let op_token = self.match_any(Operator::binary_ops_by_precedence(precedence));
                let op = if let TokenType::Operator(op) = op_token.token_type {
                    op
                } else {
                    panic!("Parse Error: at {}", self.span())
                };

                let rhs = self.parse_expression_by_precedence(scope.clone(), precedence - 1);
                let span = expr.span + rhs.span;
                expr = AstNode::new_binary_expression(
                    Box::new(expr),
                    op,
                    Box::new(rhs),
                    scope.clone(),
                    span,
                );
            }

            expr
        }
    }

    fn parse_expression_term(&mut self, scope: ScopeRef) -> AstNode {
        if self.matches(TokenTypeMatch::Identifier) {
            self.parse_identifier_expression(scope)
        } else if self.matches(TokenTypeMatch::Integer) {
            self.parse_integer(scope)
        } else if self.matches(TokenTypeMatch::Float) {
            self.parse_float(scope)
        } else if self.matches(TokenTypeMatch::String) {
            self.parse_string(scope)
        } else if self.matches_any(Operator::all_unary_ops()) {
            let op_token = self.match_any(Operator::all_unary_ops());
            let op = if let TokenType::Operator(op) = op_token.token_type {
                op
            } else {
                panic!("Parse Error: at {}", self.span())
            };

            let expr = self.parse_expression(scope.clone());
            let span = op_token.span + expr.span;
            AstNode::new_unary_expression(op, Box::new(expr), scope, span)
        } else if self.matches(Punctuation::OpenParen) {
            self.match_token(Punctuation::OpenParen);
            let expr = self.parse_expression(scope);
            self.match_token(Punctuation::CloseParen);
            expr
        } else {
            // TODO function calls and array access
            panic!("Parse Error: at {}", self.span())
        }
    }

    fn parse_identifier_expression(&mut self, scope: ScopeRef) -> AstNode {
        let identifier = self.parse_identifier(scope.clone(), true);
        let span = identifier.span.clone();
        AstNode::new_identifier_expression(Box::new(identifier), scope, span)
    }

    fn parse_integer(&mut self, scope: ScopeRef) -> AstNode {
        let token = self.match_token(TokenTypeMatch::Integer);
        match token.token_type {
            TokenType::Integer(value) => {
                AstNode::new_integer_expression(value, scope, token.span.clone())
            }
            _ => panic!("Parse Error: at {}", self.span()),
        }
    }

    fn parse_float(&mut self, scope: ScopeRef) -> AstNode {
        let token = self.match_token(TokenTypeMatch::Float);
        match token.token_type {
            TokenType::Float(value) => {
                AstNode::new_float_expression(value, scope, token.span.clone())
            }
            _ => panic!("Parse Error: at {}", self.span()),
        }
    }

    fn parse_string(&mut self, scope: ScopeRef) -> AstNode {
        let token = self.match_token(TokenTypeMatch::String);
        match token.token_type {
            TokenType::String(value) => {
                AstNode::new_string_expression(value, scope, token.span.clone())
            }
            _ => panic!("Parse Error: at {}", self.span()),
        }
    }

    fn parse_type(&mut self) -> PineType {
        let span = self.span();
        match self.token_type() {
            TokenType::Keyword(keyword) => {
                self.index += 1;
                match keyword {
                    Keyword::Int => PineType::Integer,
                    Keyword::Float => PineType::Float,
                    Keyword::String => PineType::String,
                    _ => panic!("Parse Error: at {}", span),
                }
            }
            TokenType::Punctuation(p) => {
                match p {
                    Punctuation::OpenBracket => {
                        self.index += 1;
                        let elem_type = self.parse_type();
                        self.match_token(Punctuation::OpenBracket);
                        PineType::List(Box::new(elem_type))
                    }
                    Punctuation::OpenParen => {
                        // TODO parse function type
                        panic!("Parse Error: at {}", span)
                    }
                    _ => panic!("Parse Error: at {}", span),
                }
            }
            TokenType::Identifier(_) => {
                self.index += 1;
                PineType::Void // TODO handle user defined types
            }
            _ => panic!("Parse Error: at {}", span),
        }
    }

    fn match_token<T>(&mut self, token_type: T) -> Token
    where
        T: TokenMatch + Copy + Debug,
    {
        let token = self.token();
        if token_type.matches(&token.token_type) {
            self.index += 1;
            return token;
        }

        panic!("parse error at {} expected {:?} found {:?}", token.span, token_type, token.token_type)
    }

    fn match_any<T>(&mut self, token_types: Vec<T>) -> Token
    where
        T: TokenMatch + Copy + Debug,
    {
        for token_type in token_types {
            if self.matches(token_type) {
                return self.match_token(token_type);
            }
        }

        panic!("Parse Error: at {}", self.span())
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
        } else if self.matches_any(
            Operator::all_unary_ops()
                .into_iter()
                .map(|o| o)
                .collect(),
        ) {
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
