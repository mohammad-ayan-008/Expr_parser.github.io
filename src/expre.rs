use std::{
    alloc::GlobalAlloc,
    cell::RefCell,
    cmp::Ordering,
    net,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use crate::token::{LiteralValue, Token, TokenType};

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

#[derive(Clone, Debug)]
pub enum Literal_Value {
    Number(f64),
    Boolean(bool),
    String(Rc<RefCell<String>>),
    Nil,
}
impl From<LiteralValue> for Literal_Value {
    fn from(value: LiteralValue) -> Self {
        match value {
            LiteralValue::NUMBER(a) => Literal_Value::Number(a),
            LiteralValue::STRING(a) => Literal_Value::String(a.clone()),
        }
    }
}
impl Literal_Value {
    pub fn isfalsly(&self) -> Self {
        match self {
            Self::String(a) => {
                if a.clone().borrow().is_empty() {
                    Literal_Value::Boolean(true)
                } else {
                    Literal_Value::Boolean(false)
                }
            }
            Self::Number(x) => {
                if *x == 0.0 {
                    Literal_Value::Boolean(true)
                } else {
                    Literal_Value::Boolean(false)
                }
            }
            Self::Boolean(x) => {
                if *x {
                    Literal_Value::Boolean(false)
                } else {
                    Literal_Value::Boolean(true)
                }
            }
            Self::Nil => Literal_Value::Boolean(true),
        }
    }
}
#[derive(Debug)]
pub enum Expr {
    Literal {
        value: Literal_Value,
    },
    Group {
        expr: Box<Expr>,
    },
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Variable{
        name:String
    }
}

impl Expr {
    pub fn expr(&self) -> Result<Literal_Value, String> {
        match self {
            Expr::Variable { name }=>{
                println!("Searching variable");
                Ok(Literal_Value::Nil)
            }
            Expr::Group { expr } => expr.expr(),
            Expr::Unary { op, right } => {
                let expr = right.expr()?;
                match (expr, &op.token_type) {
                    (any, TokenType::Not) => Ok(any.isfalsly()),
                    (Literal_Value::Number(a), TokenType::Sub) => Ok(Literal_Value::Number(-a)),
                    _ => todo!(),
                }
            }
            Expr::Literal { value } => Ok(value.clone()),

            Expr::Binary { left, op, right } => {
                let val = left.expr()?;
                let val2 = right.expr()?;

                match (val, &op.token_type, val2) {
                    (Literal_Value::Number(a), TokenType::Sub, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Number(a - b))
                    }
                    (Literal_Value::Number(a), TokenType::Mul, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Number(a * b))
                    }
                    (Literal_Value::Number(a), TokenType::Div, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Number(a / b))
                    }
                    (Literal_Value::Number(a), TokenType::Add, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Number(a + b))
                    }

                    (Literal_Value::Number(a), TokenType::Greater, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Boolean(a > b))
                    }
                    (
                        Literal_Value::Number(a),
                        TokenType::GreaterEqual,
                        Literal_Value::Number(b),
                    ) => Ok(Literal_Value::Boolean(a >= b)),
                    (Literal_Value::Number(a), TokenType::Lesser, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Boolean(a < b))
                    }
                    (
                        Literal_Value::Number(a),
                        TokenType::LesserEqual,
                        Literal_Value::Number(b),
                    ) => Ok(Literal_Value::Boolean(a <= b)),
                    (
                        Literal_Value::Number(a),
                        TokenType::EqualEquals,
                        Literal_Value::Number(b),
                    ) => Ok(Literal_Value::Boolean(a == b)),
                    (Literal_Value::Number(a), TokenType::NotEqual, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Boolean(a != b))
                    }
                    (
                        Literal_Value::Boolean(a),
                        TokenType::EqualEquals,
                        Literal_Value::Boolean(b),
                    ) => Ok(Literal_Value::Boolean(a == b)),
                    (Literal_Value::Boolean(a), TokenType::NotEqual, Literal_Value::Boolean(b)) => {
                        Ok(Literal_Value::Boolean(a != b))
                    }

                    (Literal_Value::String(a), TokenType::NotEqual, Literal_Value::String(b)) => {
                        Ok(Literal_Value::Boolean(a != b))
                    }
                    (
                        Literal_Value::String(a),
                        TokenType::EqualEquals,
                        Literal_Value::String(b),
                    ) => Ok(Literal_Value::Boolean(a == b)),
                    (Literal_Value::Nil, TokenType::EqualEquals, Literal_Value::Nil) => {
                        Ok(Literal_Value::Boolean(true))
                    }
                    (Literal_Value::Nil, TokenType::NotEqual, Literal_Value::Nil) => {
                        Ok(Literal_Value::Boolean(false))
                    }

                    (Literal_Value::String(val_str), TokenType::Add, a) => {
                        let str = val_str.clone();
                        match a {
                            Literal_Value::Number(int_val) => {
                                str.borrow_mut().push_str(&int_val.to_string());
                                Ok(Literal_Value::String(str))
                            }
                            Literal_Value::String(d) => {
                                str.borrow_mut().push_str(&d.clone().borrow());
                                Ok(Literal_Value::String(str))
                            }
                            Literal_Value::Boolean(d) => {
                                str.borrow_mut().push_str(&d.to_string());
                                Ok(Literal_Value::String(str))
                            }
                            a => Err(format!("Not implemented for {:?}", a)),
                        }
                    }
                    a => Err(format!("not yet impl for {:?}", a)),
                }
            }
        }
    }
}
