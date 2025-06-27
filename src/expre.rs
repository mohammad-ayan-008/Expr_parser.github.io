use std::{
    alloc::GlobalAlloc,
    cell::RefCell,
    cmp::Ordering,
    net,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

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

#[derive(Clone, Debug)]
pub enum Literal_Value {
    Number(f64),
    Boolean(bool),
    String(Rc<RefCell<String>>),
    Nil,
}
impl From<&crate::lexer::LiteralValue> for Literal_Value {
    fn from(value: &crate::lexer::LiteralValue) -> Self {
        match value {
            LiteralValue::NUMBER(a) => Literal_Value::Number(*a),
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
}

impl Expr {
    pub fn expr(&self) -> Result<Literal_Value, String> {
        match self {
            Expr::Group { expr } => expr.expr(),
            Expr::Unary { op, right } => {
                let expr = right.expr()?;
                match (expr, op) {
                    (any, Token::Not) => Ok(any.isfalsly()),
                    (Literal_Value::Number(a), Token::Sub) => Ok(Literal_Value::Number(-a)),
                    _ => todo!(),
                }
            }
            Expr::Literal { value } => Ok(value.clone()),

            Expr::Binary { left, op, right } => {
                let val = left.expr()?;
                let val2 = right.expr()?;

                match (val, op, val2) {
                    (Literal_Value::Number(a), Token::Sub, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Number(a - b))
                    }
                    (Literal_Value::Number(a), Token::Mul, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Number(a * b))
                    }
                    (Literal_Value::Number(a), Token::Div, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Number(a / b))
                    }
                    (Literal_Value::Number(a), Token::Add, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Number(a + b))
                    }

                    (Literal_Value::Number(a), Token::Greater, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Boolean(a > b))
                    }
                    (Literal_Value::Number(a), Token::GreaterEqual, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Boolean(a >= b))
                    }
                    (Literal_Value::Number(a), Token::Lesser, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Boolean(a < b))
                    }
                    (Literal_Value::Number(a), Token::LesserEqual, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Boolean(a <= b))
                    }
                    (Literal_Value::Number(a), Token::EqualEquals, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Boolean(a == b))
                    }
                    (Literal_Value::Number(a), Token::NotEqual, Literal_Value::Number(b)) => {
                        Ok(Literal_Value::Boolean(a != b))
                    }
                    (Literal_Value::Boolean(a), Token::EqualEquals, Literal_Value::Boolean(b)) => {
                        Ok(Literal_Value::Boolean(a == b))
                    }
                    (Literal_Value::Boolean(a), Token::NotEqual, Literal_Value::Boolean(b)) => {
                        Ok(Literal_Value::Boolean(a != b))
                    }

                    (Literal_Value::String(a), Token::NotEqual, Literal_Value::String(b)) => {
                        Ok(Literal_Value::Boolean(a != b))
                    }
                    (Literal_Value::String(a), Token::EqualEquals, Literal_Value::String(b)) => {
                        Ok(Literal_Value::Boolean(a == b))
                    }
                    (Literal_Value::Nil, Token::EqualEquals, Literal_Value::Nil) => {
                        Ok(Literal_Value::Boolean(true))
                    }
                    (Literal_Value::Nil, Token::NotEqual, Literal_Value::Nil) => {
                        Ok(Literal_Value::Boolean(false))
                    }

                    (Literal_Value::String(val_str), Token::Add, a) => {
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
