use std::{any::Any, env::set_var, iter::Peekable};

use crate::{
    stmt::Statement, expre::{Expr, Literal_Value}, token::{self, Token, TokenType}
};
/*


expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;

*/

/*program        → statement* EOF ;
:
statement      → exprStmt| printStmt ;

exprStmt       → expression ";" ;
printStmt      → "print" "(" expression  ")" ";" ;
*/

pub struct Parser {
    pub tokens: Vec<Token>,
    pub counter: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, counter: 0 }
    }

    pub fn parse(&mut self)->Result<Vec<Statement>,String>{
       let mut statements = vec![];
        while !self.is_End(){
            statements.push(self.statements()?);
        }
        Ok(statements)
    }

    pub fn statements(&mut self)->Result<Statement,String>{
        if self.match_token(&[TokenType::Print]){
            self.printStatement()
        }else {
            self.expressionStatement()
        }
    }

    pub fn printStatement(&mut self)->Result<Statement,String>{
        self.consume(&TokenType::LeftParen, "Expected ( after print keyword")?;
        let expr =self.expression()?;
        self.consume(&TokenType::RightParen, "Expected ) after the expression")?;
        self.consume(&TokenType::Semicoln, "Expected ; after the expression")?;
        Ok(Statement::Print { expession: expr })
    }

    pub fn expressionStatement(&mut self)->Result<Statement,String>{
        let expr = self.expression()?;
        self.consume(&TokenType::Semicoln, "Expected ; after the expression")?;
        Ok(Statement::Expression { exp: expr })
    }

    pub fn expression(&mut self) -> Result<Expr, String> {
        if !self.is_End(){
          self.equality()
        }else {
            //Empty Input
            Err("Empty Input".to_string())
        }
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut lhs = self.comparison();
        while self.match_token(&[TokenType::NotEqual, TokenType::EqualEquals]) {
            // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
            let token = self.previous().clone();
            let rhs = self.comparison()?;
            lhs = Ok(Expr::Binary {
                left: Box::new(lhs?),
                op: token,
                right: Box::new(rhs),
            })
        }

        lhs
    }

    pub fn comparison(&mut self) -> Result<Expr, String> {
        let mut lhs = self.term();
        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Lesser,
            TokenType::LesserEqual,
        ]) {
            //comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
            let token = self.previous().clone();
            let right = self.term()?;
            lhs = Ok(Expr::Binary {
                left: Box::new(lhs?),
                op: token,
                right: Box::new(right),
            })
        }
        lhs
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut lhs = self.factor();
        while self.match_token(&[TokenType::Add, TokenType::Sub]) {
            // term           → factor ( ( "-" | "+" ) factor )* ;
            let token = self.previous().clone();
            let right = self.factor()?;
            lhs = Ok(Expr::Binary {
                left: Box::new(lhs?),
                op: token,
                right: Box::new(right),
            })
        }
        lhs
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut lhs = self.unary();
        while self.match_token(&[TokenType::Div, TokenType::Mul]) {
            // factor         → unary ( ( "/" | "*" ) unary )* ;
            let token = self.previous().clone();
            let right = self.unary()?;
            lhs = Ok(Expr::Binary {
                left: Box::new(lhs?),
                op: token,
                right: Box::new(right),
            })
        }
        lhs
    }
    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(&[TokenType::Not, TokenType::Sub]) {
            // factor         → unary ( ( "/" | "*" ) unary )* ;
            let token = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                op: token,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek().clone();
        match token.token_type {
            TokenType::False => {
                self.advance();
                Ok(Expr::Literal {
                    value: crate::expre::Literal_Value::Boolean(false),
                })
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Literal {
                    value: crate::expre::Literal_Value::Boolean(true),
                })
            }
            TokenType::Literal { value } => {
                let val = Expr::Literal {
                    value: Literal_Value::from(value),
                };
                self.advance();
                Ok(val)
            }
            TokenType::Nil => {
                self.advance();
                Ok(Expr::Literal {
                    value: crate::expre::Literal_Value::Nil,
                })
            }
            TokenType::Identifier { value }=>{
                self.advance();
                Ok(Expr::Variable { name: value })
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression();
                self.consume(&TokenType::RightParen, "Expected ) after expression")?;
                Ok(Expr::Group {
                    expr: Box::new(expr?),
                })
            }
            _ => Err(format!("Invalid token {:?}", token)),
        }
    }

    fn peek(&self) -> &Token{
        if self.counter >= self.tokens.len(){
            return &self.tokens[self.tokens.len()-1];
        }
        &self.tokens[self.counter]
    }

    fn advance(&mut self) -> Token {
        let token = &self.tokens[self.counter];
        self.counter += 1;
        token.clone()
    }

    fn match_token(&mut self, tokens: &[TokenType]) -> bool {
        for i in tokens {
            if self.check(i) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, tokens: &TokenType) -> bool {
        if self.is_End() {
            return false;
        };
        self.peek().token_type == *tokens
    }

    fn is_End(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
    fn consume(&mut self, token: &TokenType, msg: &str) -> Result<&Token, String> {
        if self.check(token) {
            self.advance();
            Ok(self.previous())
        } else {
            Err(format!("{} at line {}", msg,self.peek().line))
        }
    }

    fn previous(&mut self) -> &Token {
        let token = self.tokens.get(self.counter - 1).unwrap();
        token
    }
}
