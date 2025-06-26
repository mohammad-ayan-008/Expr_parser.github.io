use std::{cmp::Ordering, net, ops::{Add, Div, Mul, Sub}, rc::Rc};

use crate::lexer::{LiteralValue, Token};

/*
expression     → literal
               | unary
               | binary
               | grouping ;

literal        → NUMBER | STRING | "true" | "false" | "nil" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
               | "+"  | "-"  | "*" | "/" ;
*/

#[derive(Clone,Debug)]
pub enum Literal_Value {
    Number(f64),
    Boolean(bool),
    String(Rc<String>),
    Nil,
}
impl PartialEq for Literal_Value{
    fn eq(&self, other: &Self) -> bool {
        match (self,other) {
            (Literal_Value::Number(a),Literal_Value::Number(b))=> a==b,
            (Literal_Value::Boolean(a),Literal_Value::Boolean(b))=> a==b,
            (Literal_Value::String(a),Literal_Value::String(b))=> a == b,
            (Literal_Value::String(a),Literal_Value::Nil)=> {
                false
            }
            a=>panic!("not implemented for {:?}",a)
        }
    }
}

impl PartialOrd for Literal_Value{

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { 
        match (self,other) { 
            (Literal_Value::Boolean(a),Literal_Value::Boolean(b)) => a.partial_cmp(b),
            (Literal_Value::Number(a),Literal_Value::Number(b)) => a.partial_cmp(b),
            a=>panic!("Not implemented for type {:?}",a)
        }
    }
    
}
impl From<&crate::lexer::LiteralValue> for Literal_Value{
    fn from(value: &crate::lexer::LiteralValue) -> Self {
         match value {
             LiteralValue::NUMBER(a)=>Literal_Value::Number(*a),
             LiteralValue::STRING(a)=>Literal_Value::String(a.clone()),
         }
    }
}
impl Literal_Value{
    pub fn isfalsly(&self)->Self{
        match self {
            Self::String(a)=>{
                if a.is_empty() {
                    Literal_Value::Boolean(false)
                }else {
                    Literal_Value::Boolean(true)
                }
            }
            Self::Number(x)=>{
                if *x==0.0 {
                     Literal_Value::Boolean(true)
                }else {
                     Literal_Value::Boolean(false)
                }
            }
            Self::Boolean(x)=>{
                if *x {
                    Literal_Value::Boolean(false)
                }else {
                    Literal_Value::Boolean(true)
                }
            }
            Self::Nil=>Literal_Value::Boolean(true)
        }
    }
}
impl Add for Literal_Value {
    type Output = Literal_Value;
    fn add(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (Literal_Value::Number(a),Literal_Value::Number(b))=> Literal_Value::Number(a+b),
            (a,b)=>panic!("cannot add {:?} and {:?}",a,b)
        }
    }
}
impl Sub for Literal_Value {
    type Output = Literal_Value;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (Literal_Value::Number(a),Literal_Value::Number(b))=> Literal_Value::Number(a-b),
            (a,b)=>panic!("cannot add {:?} and {:?}",a,b)
        }
    }
}
impl Div for Literal_Value {
    type Output = Literal_Value;
    fn div(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (Literal_Value::Number(a),Literal_Value::Number(b))=> Literal_Value::Number(a/b),
            (a,b)=>panic!("cannot add {:?} and {:?}",a,b)
        }
    }
}
impl Mul for Literal_Value {
    type Output = Literal_Value;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (Literal_Value::Number(a),Literal_Value::Number(b))=> Literal_Value::Number(a*b),
            (a,b)=>panic!("cannot add {:?} and {:?}",a,b)
        }
    }
}
#[derive(Debug)]
pub enum Expr {
    Literal{
        value:Literal_Value
    },
    Group{
        expr:Box<Expr>
    },
    Unary{
        op:Token,
        right:Box<Expr>
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn expr(&self) -> Literal_Value{
        match self {
            Expr::Group { expr }=>{
                expr.expr()
            }
            Expr::Unary { op, right }=>{
                let expr = right.expr();
                match (expr,op) {
                    (any,Token::Not)=>{
                        any.isfalsly()
                    },
                    (Literal_Value::Number(a),Token::Sub)=>{
                        Literal_Value::Number(-a)
                    },
                    _=>todo!()
                }
            }
            Expr::Literal{value}=> {
               value.clone() 
            },
            Expr::Binary { left, op, right } => {
                let val = left.expr();
                let val2 = right.expr();

                match op {
                    Token::Sub => val - val2,
                    Token::Add => val + val2,
                    Token::Div => val / val2,
                    Token::Mul => val * val2,
                    Token::EqualEquals => Literal_Value::Boolean(val == val2),
                    Token::Lesser => Literal_Value::Boolean(val < val2),
                    Token::LesserEqual => Literal_Value::Boolean(val <= val2),
                    Token::Greater => Literal_Value::Boolean(val > val2),
                    Token::GreaterEqual => Literal_Value::Boolean(val >= val2),
                    Token::NotEqual => Literal_Value::Boolean(val != val2),
                    _ => todo!(),
                }
            }
        }
    }
}
