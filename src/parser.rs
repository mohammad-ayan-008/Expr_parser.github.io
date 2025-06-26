use std::{env::set_var, iter::Peekable};

use crate::{expre::Expr, lexer::Token};
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
pub struct Parser {
    pub tokens: Vec<Token>,
    pub counter: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, counter: 0 }
    }

    pub fn expression(&mut self)->Expr{
        self.equality()
    }

    fn equality(&mut self)->Expr{
        let mut lhs  = self.comparison(); 
        while self.match_token(&[Token::NotEqual,Token::EqualEquals]) {  // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
            let token = self.previous().clone();
            let rhs = self.comparison();
            lhs = Expr::Binary { left: Box::new(lhs), op: token, right: Box::new(rhs)}
        }

        lhs
    }


        
    pub fn comparison(&mut self) -> Expr {
        let mut lhs = self.term();
        while self.match_token(&[Token::Greater, Token::GreaterEqual,Token::Lesser,Token::LesserEqual]) {//comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
            let token = self.previous().clone();
            let right = self.term();
            lhs = Expr::Binary {
                left: Box::new(lhs),
                op: token,
                right: Box::new(right),
            }
        }
        lhs
    }

    fn term(&mut self) -> Expr {
        let mut lhs = self.factor();
        while self.match_token(&[Token::Add, Token::Sub]) {// term           → factor ( ( "-" | "+" ) factor )* ;
            let token = self.previous().clone();
            let right = self.factor();
            lhs = Expr::Binary {
                left: Box::new(lhs),
                op: token,
                right: Box::new(right),
            }
        }
        lhs
    }

    fn factor(&mut self) -> Expr {
        let mut lhs = self.unary();
        while self.match_token(&[Token::Div, Token::Mul]) { // factor         → unary ( ( "/" | "*" ) unary )* ;
            let token = self.previous().clone();
            let right = self.unary();
            lhs = Expr::Binary {
                left: Box::new(lhs),
                op: token,
                right: Box::new(right),
            }
        }
        lhs
    }
    fn unary(&mut self) -> Expr {
        if self.match_token(&[Token::Not, Token::Sub]) {// factor         → unary ( ( "/" | "*" ) unary )* ;
            let token = self.previous().clone();
            let right = self.primary();
             return Expr::Unary { op: token, right: Box::new(right)
            }
        }
        self.primary()
    }

    fn primary(&mut self)->Expr{
        let token = self.peek();
        match token {
            Token::False => {
                self.advance();
                Expr::Literal { value: crate::expre::Literal_Value::Boolean(false)}
            },
            Token::True => {
                self.advance();
                Expr::Literal { value: crate::expre::Literal_Value::Boolean(true) }
            },
            Token::Literal{value} => {
                let val =Expr::Literal { value: crate::expre::Literal_Value::from(value) };
                self.advance();
                val
            },
            Token::Nil=>{
                self.advance();
                Expr::Literal { value: crate::expre::Literal_Value::Nil }
            }
            Token::LeftParen=>{
                 self.advance();
                 let expr = self.expression();
                 self.consume(&Token::RightParen, "Expected ) after expression"); 
                 Expr::Group { expr: Box::new(expr) }
            }

            a=>{
                panic!("Invalid token {:?}",a)
            }
        }
    }



    fn peek(&self) -> &Token {
        &self.tokens[self.counter]
    }

    fn advance(&mut self) -> Token {
        let token = &self.tokens[self.counter];
        self.counter += 1;
        token.clone()
    }

    fn match_token(&mut self, tokens: &[Token]) -> bool {
        for i in tokens {
            if self.check(i) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, tokens: &Token) -> bool {
        if self.is_End() {
            return false;
        };
        *self.peek() == *tokens
    }

    fn is_End(&self) -> bool {
        *self.peek() == Token::Eof
    }
    fn consume(&mut self, token: &Token, msg: &str) -> &Token {
        if self.check(token) {
            self.advance();
            self.previous()
        } else {
            eprintln!("{}", msg);
            &Token::Eof
        }
    }

    fn previous(&mut self) -> &Token {
        let token =self.tokens.get(self.counter - 1).unwrap();
        token
    }
}
