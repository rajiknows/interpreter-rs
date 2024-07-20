use super::ast::{Expr, Stmt};
use super::token::Token;
use anyhow::{anyhow, Result};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    curr_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let curr_token = tokens.get(0).cloned().unwrap_or(Token::Eof);
        let peek_token = tokens.get(1).cloned().unwrap_or(Token::Eof);

        Parser {
            tokens,
            position: 0,
            curr_token,
            peek_token,
        }
    }

    fn next_token(&mut self) -> Result<()> {
        self.position += 1;
        self.curr_token = self.peek_token.clone();
        self.peek_token = self
            .tokens
            .get(self.position + 1)
            .cloned()
            .unwrap_or(Token::Eof);
        Ok(())
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while self.curr_token != Token::Eof {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
            self.next_token()?;
        }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.curr_token {
            Token::Let => self.parse_let_statement(),
            Token::Print => self.parse_print_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Stmt> {
        self.next_token()?; // Skip 'let'

        let name = if let Token::Ident(ref name) = self.curr_token {
            name.clone()
        } else {
            return Err(anyhow!("expected identifier, got {:?}", self.curr_token));
        };

        self.next_token()?; // Skip identifier

        if self.curr_token != Token::Assign {
            return Err(anyhow!("expected '=', got {:?}", self.curr_token));
        }

        self.next_token()?; // Skip '='

        let value = self.parse_expression()?;

        if self.peek_token == Token::Semicolon {
            self.next_token()?;
        }

        Ok(Stmt::Let(name, value))
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.parse_expression()?;

        if self.peek_token == Token::Semicolon {
            self.next_token()?;
        }

        Ok(Stmt::Expr(expr))
    }

    fn parse_expression(&mut self) -> Result<Expr> {
        self.parse_infix_expression()
    }

    fn parse_infix_expression(&mut self) -> Result<Expr> {
        let mut left = self.parse_primary()?;

        while matches!(
            self.peek_token,
            Token::Plus | Token::Dash | Token::Asterisk | Token::ForwardSlash
        ) {
            let op = match &self.peek_token {
                Token::Plus => "+",
                Token::Dash => "-",
                Token::Asterisk => "*",
                Token::ForwardSlash => "/",
                _ => unreachable!(),
            }
            .to_string();

            self.next_token()?;
            self.next_token()?; // move to the right expression

            let right = self.parse_primary()?;

            left = Expr::Infix(Box::new(left), op, Box::new(right));
        }

        Ok(left)
    }

    fn parse_print_statement(&mut self) -> Result<Stmt> {
        self.next_token()?;
        if self.curr_token != Token::Lparen {
            return Err(anyhow!("expected '(', got {:?}", self.curr_token));
        }

        self.next_token()?;

        let value = self.parse_expression()?;
        self.next_token()?;

        if self.curr_token != Token::Rparen {
            return Err(anyhow!("expected ')', got {:?}", self.curr_token));
        }

        self.next_token()?;

        if self.peek_token == Token::Semicolon {
            self.next_token()?;
        }

        Ok(Stmt::Print(value))
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        match &self.curr_token {
            Token::Int(value) => Ok(Expr::Int(value.parse()?)),
            Token::Ident(name) => Ok(Expr::Ident(name.clone())),
            _ => Err(anyhow!("unexpected token: {:?}", self.curr_token)),
        }
    }
}
