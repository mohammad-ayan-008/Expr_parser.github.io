use std::{
    cell::RefCell, collections::HashMap, iter::Peekable, net::ToSocketAddrs, panic, rc::Rc,
    str::Chars, vec,
};
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
pub enum Token {
    Literal { value: LiteralValue },
    Add,
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
}

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    pub tokens: Vec<Token>,
    keywords: HashMap<String, Token>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            line: 0,
            tokens: vec![],
            source: source.chars().peekable(),
            keywords: Self::init_keywords(),
        }
    }

    fn init_keywords() -> HashMap<String, Token> {
        let mut map = HashMap::new();
        map.insert("false".to_string(), Token::False);
        map.insert("true".to_string(), Token::True);
        map.insert("nil".to_string(), Token::Nil);
        map
    }
    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn next(&mut self) -> Option<char> {
        self.source.next()
    }
    pub fn lexe(&mut self) -> Result<&Vec<Token>, String> {
        while let Some(a) = self.next() {
            match Some(a) {
                Some('(') => {
                    self.add_token(Token::LeftParen);
                }
                Some('+') => {
                    self.add_token(Token::Add);
                }
                Some('-') => {
                    self.add_token(Token::Sub);
                }
                Some('/') => {
                    self.add_token(Token::Div);
                }
                Some('*') => {
                    self.add_token(Token::Mul);
                }
                Some('"') => {
                    let mut str = String::new();
                    let mut terminated = false;
                    while let Some(a) = self.peek() {
                        if a != &'"' {
                            str.push(self.next().unwrap());
                        } else {
                            terminated = true;
                            self.next().unwrap();
                            break;
                        }
                    }
                    if !terminated {
                        return Err("non terminated String".to_string());
                    }
                    self.add_token(Token::Literal {
                        value: LiteralValue::from(str),
                    });
                }
                Some(')') => {
                    self.add_token(Token::RightParen);
                }
                Some(a @ '=') => {
                    if self.peek() == Some(&'=') {
                        self.add_token(Token::EqualEquals);
                        self.next();
                    } else {
                        return Err(format!("Invalid Token {} {:?}", a, self.peek()));
                    }
                }
                Some('>') => {
                    if self.peek() == Some(&'=') {
                        self.add_token(Token::GreaterEqual);
                        self.next();
                    } else {
                        self.add_token(Token::Greater);
                    }
                }
                Some('!') => {
                    if self.peek() == Some(&'=') {
                        self.add_token(Token::NotEqual);
                        self.next();
                    } else {
                        self.add_token(Token::Not);
                    }
                }

                Some('<') => {
                    if self.peek() == Some(&'=') {
                        self.add_token(Token::LesserEqual);
                        self.next();
                    } else {
                        self.add_token(Token::Lesser);
                    }
                }
                Some(num @ '0'..='9') => {
                    let mut str = String::new();
                    str.push(num);
                    while let Some(a) = self.peek() {
                        if a.is_ascii_digit() || a == &'.' {
                            str.push(self.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    self.add_token(Token::Literal {
                        value: LiteralValue::from(str.parse::<f64>().unwrap()),
                    });
                }
                c if c.unwrap().is_ascii_alphabetic() => {
                    let mut str = String::new();
                    str.push(c.unwrap());
                    while let Some(a) = self.next() {
                        if a.is_ascii_alphanumeric() {
                            str.push(a);
                        } else {
                            break;
                        }
                    }
                    let key = self.keywords.get(&str);
                    if let Some(a) = key {
                        self.add_token(a.clone());
                    } else {
                        return Err(format!("not an identifier {}", str));
                    }
                }
                Some('\n') => {
                    self.line += 1;
                }
                Some(' ') | Some('\t') => {}
                a => return Err(format!("{:?}", a)),
            }
        }
        self.tokens.push(Token::Eof);
        Ok(&self.tokens)
    }
    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     fn tokenize(input: &str) -> Vec<Token> {
//         let mut lexer = Lexer::new(input);
//         lexer.lexe().clone()
//     }
//
//     #[test]
//     fn test_basic_math() {
//         let tokens = tokenize("(1+2)");
//         assert_eq!(
//             tokens,
//             vec![
//                 Token::LeftParen,
//                 Token::Number(1.0),
//                 Token::Add,
//                 Token::Number(2.0),
//                 Token::RightParen,
//                 Token::Eof
//             ]
//         );
//     }
//
//     #[test]
//     fn test_logical_operators() {
//         let tokens = tokenize("!(1>1+2)");
//         assert_eq!(
//             tokens,
//             vec![
//                 Token::Not,
//                 Token::LeftParen,
//                 Token::Number(1.0),
//                 Token::Greater,
//                 Token::Number(1.0),
//                 Token::Add,
//                 Token::Number(2.0),
//                 Token::RightParen,
//                 Token::Eof
//             ]
//         );
//     }
//
//     #[test]
//     fn test_equality_and_inequality() {
//         let tokens = tokenize("1==2!=3");
//         assert_eq!(
//             tokens,
//             vec![
//                 Token::Number(1.0),
//                 Token::EqualEquals,
//                 Token::Number(2.0),
//                 Token::NotEqual,
//                 Token::Number(3.0),
//                 Token::Eof
//             ]
//         );
//     }
//
//     #[test]
//     fn test_greater_equal_and_lesser_equal() {
//         let tokens = tokenize("1>=2<=3");
//         assert_eq!(
//             tokens,
//             vec![
//                 Token::Number(1.0),
//                 Token::GreaterEqual,
//                 Token::Number(2.0),
//                 Token::LesserEqual,
//                 Token::Number(3.0),
//                 Token::Eof
//             ]
//         );
//     }
//
//     #[test]
//     fn test_nil_keyword() {
//         let tokens = tokenize("nil");
//         assert_eq!(tokens, vec![Token::Nil, Token::Eof]);
//     }
//
//     #[test]
//     #[should_panic(expected = "not an identifier foo")]
//     fn test_invalid_identifier() {
//         let _ = tokenize("foo");
//     }
//
//     #[test]
//     #[should_panic(expected = "Invalid Token")]
//     fn test_single_equal_panics() {
//         let _ = tokenize("1=2");
//     }
//
//     #[test]
//     fn test_multiple_lines() {
//         let mut lexer = Lexer::new("1\n2\n3");
//         let _ = lexer.lexe();
//         assert_eq!(lexer.line, 2);
//     }
//
//     #[test]
//     fn test_less_than() {
//         let tokens = tokenize("1<2");
//         assert_eq!(
//             tokens,
//             vec![
//                 Token::Number(1.0),
//                 Token::Lesser,
//                 Token::Number(2.0),
//                 Token::Eof
//             ]
//         );
//     }
//
//     #[test]
//     fn test_less_than_equal() {
//         let tokens = tokenize("1<=2");
//         assert_eq!(
//             tokens,
//             vec![
//                 Token::Number(1.0),
//                 Token::LesserEqual,
//                 Token::Number(2.0),
//                 Token::Eof
//             ]
//         );
//     }
//
//     #[test]
//     fn test_mix_with_math() {
//         let tokens = tokenize("(3+2)<(4*1)");
//         assert_eq!(
//             tokens,
//             vec![
//                 Token::LeftParen,
//                 Token::Number(3.0),
//                 Token::Add,
//                 Token::Number(2.0),
//                 Token::RightParen,
//                 Token::Lesser,
//                 Token::LeftParen,
//                 Token::Number(4.0),
//                 Token::Mul,
//                 Token::Number(1.0),
//                 Token::RightParen,
//                 Token::Eof
//             ]
//         );
//     }
//
//     #[test]
//     fn test_less_than_at_end() {
//         let tokens = tokenize("9<");
//         assert_eq!(
//             tokens,
//             vec![
//                 Token::Number(9.0),
//                 Token::Lesser,
//                 Token::Eof
//             ]
//         );
//     }
// }
