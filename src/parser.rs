use std::env::set_var;

use crate::{expre::Expr, lexer::Token};
/*
```
Expr   → Expr "+" Term
       | Expr "-" Term
       | Term

Term   → Term "*" Factor
       | Term "/" Factor
       | Factor

Factor → "(" Expr ")"
       | number```

*/


pub struct Parser{
    tokens:Vec<Token>,
    counter:usize
}

impl Parser {
    
    pub fn new(tokens:Vec<Token>)->Self{
        Self { tokens ,counter:0}
    }

    pub fn parse_expr(&mut self)->Expr{

       let mut lhs = self.term();
        while self.match_token(&[Token::Add,Token::Sub]) {
            let token = self.previous();
            let right = self.term();
            lhs = Expr::Binary { left: Box::new(lhs), op: token, right: Box::new(right) }
        }
        lhs
    }

    
    fn term(&mut self)->Expr{
        let mut term = self.factor();
        while self.match_token(&[Token::Mul,Token::Div]) {
            let token = self.previous();
            let right = self.factor();
            term = Expr::Binary { left: Box::new(term), op: token, right: Box::new(right) }
        }
        term
    }


      fn factor(&mut self)->Expr{
        match self.advance() {
            Token::Number(a) =>{
                Expr::Number(a.to_digit(10).unwrap() as f64)
            },
            Token::LeftParen =>{
                let expr = self.parse_expr();
                self.consume(Token::RightParen, "Expected ) after a expression");
                Expr::Group(Box::new(expr))
            }
            a=> panic!("uknown symbol {:?}",a)
        }
    }

    fn peek(&self)->&Token{
        &self.tokens[self.counter]
    }

    fn advance(&mut self)->Token{ 
        let token = self.tokens[self.counter];
        self.counter +=1;
        token
    }


      fn match_token(&mut self,tokens:&[Token])->bool{
        for i in tokens{
            if self.check(*i){
                self.advance();
                return true;
            }
        }
        false
    }
    
    fn check(&self,tokens:Token)->bool{
        if  self.is_End() {return false};
        *self.peek() == tokens
    }

    fn is_End(&self)->bool{
         *self.peek() == Token::Eof
    }
    fn consume(&mut self,token:Token,msg:&str)->Token{      
        if self.check(token){
            self.advance();
            self.previous()
        }else {
            eprintln!("{}",msg);
            Token::Eof
        }
    }

    fn previous(&mut self)->Token{
        self.tokens.get(self.counter -1).copied().unwrap()
    }

}
