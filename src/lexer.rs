use std::{
    cell::RefCell, collections::HashMap, iter::Peekable, net::ToSocketAddrs, panic, rc::Rc,
    str::Chars, vec,
};

use crate::token::{LiteralValue, Token, TokenType};
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    pub tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
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

    fn init_keywords() -> HashMap<String, TokenType> {
        let mut map = HashMap::new();
        map.insert("false".to_string(), TokenType::False);
        map.insert("true".to_string(), TokenType::True);
        map.insert("nil".to_string(), TokenType::Nil);
        map.insert("print".to_string(), TokenType::Print);
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
                    self.add_token(TokenType::LeftParen);
                },
                Some(';') =>{
                    self.add_token(TokenType::Semicoln);
                }
                Some('+') => {
                    self.add_token(TokenType::Add);
                }
                Some('-') => {
                    self.add_token(TokenType::Sub);
                }
                Some('/') => {
                    self.add_token(TokenType::Div);
                }
                Some('*') => {
                    self.add_token(TokenType::Mul);
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
                    self.add_token(TokenType::Literal {
                        value: LiteralValue::from(str),
                    });
                }
                Some(')') => {
                    self.add_token(TokenType::RightParen);
                }
                Some(a @ '=') => {
                    if self.peek() == Some(&'=') {
                        self.add_token(TokenType::EqualEquals);
                        self.next();
                    } else {
                        return Err(format!("Invalid Token {} {:?}", a, self.peek()));
                    }
                }
                Some('>') => {
                    if self.peek() == Some(&'=') {
                        self.add_token(TokenType::GreaterEqual);
                        self.next();
                    } else {
                        self.add_token(TokenType::Greater);
                    }
                }
                Some('!') => {
                    if self.peek() == Some(&'=') {
                        self.add_token(TokenType::NotEqual);
                        self.next();
                    } else {
                        self.add_token(TokenType::Not);
                    }
                }

                Some('<') => {
                    if self.peek() == Some(&'=') {
                        self.add_token(TokenType::LesserEqual);
                        self.next();
                    } else {
                        self.add_token(TokenType::Lesser);
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
                    self.add_token(TokenType::Literal {
                        value: LiteralValue::from(str.parse::<f64>().unwrap()),
                    });
                }
                c if c.unwrap().is_ascii_alphabetic() => {
                    let mut str = String::new();
                    str.push(c.unwrap());
                    while let Some(a) = self.peek() {
                        if a.is_ascii_alphanumeric() {
                            str.push(self.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    let key = self.keywords.get(&str);
                    if let Some(a) = key {
                        self.add_token(a.clone());
                    } else {
                        self.add_token(TokenType::Identifier { value: str });
                    }
                }
                Some('\n') => {
                    self.line += 1;
                }
                Some(' ') | Some('\t') => {}
                a => return Err(format!("{:?}", a)),
            }
        }
        self.add_token(TokenType::Eof);
        Ok(&self.tokens)
    }
    fn add_token(&mut self, token: TokenType) {
        let token = Token {
            token_type: token,
            line: self.line,
        };
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
