use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Clone, PartialOrd, Debug)]
pub enum LiteralValue {
    NUMBER(f64),
    STRING(Rc<RefCell<String>>),
}
impl From<f64> for LiteralValue {
    fn from(value: f64) -> Self {
        LiteralValue::NUMBER(value)
    }
}
impl From<String> for LiteralValue {
    fn from(value: String) -> Self {
        LiteralValue::STRING(Rc::new(RefCell::new(value)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Literal { value: LiteralValue },
    Identifier{value:String},
    Add,
    Semicoln,
    Sub,
    Div,
    Nil,
    Greater,
    GreaterEqual,
    LesserEqual,
    Lesser,
    EqualEquals,
    Not,
    NotEqual,
    True,
    False,
    Mul,
    RightParen,
    LeftParen,
    Eof,
    Print
}
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}
