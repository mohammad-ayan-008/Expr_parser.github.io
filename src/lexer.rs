use std::{collections::HashMap, iter::Peekable, str::Chars, vec};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(char),
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
    Mul,
    RightParen,
    LeftParen,
    Eof,
}

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    keywords: HashMap<String, Token>,
    line:usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            line:0,
            tokens: vec![],
            source: source.chars().peekable(),
            keywords: Self::init_keywords(),
        }
    }

    fn init_keywords() -> HashMap<String, Token> {
        let mut map = HashMap::new();
        map.insert("nil".to_string(), Token::Nil);
        map
    }
    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn next(&mut self) -> Option<char> {
        self.source.next()
    }
    pub fn lexe(&mut self) -> &Vec<Token> {
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
                Some(')') => {
                    self.add_token(Token::RightParen);
                }
                Some(a @ '=') => {
                    if self.peek() == Some(&'=') {
                        self.add_token(Token::EqualEquals);
                        self.next();
                    } else {
                        panic!("Invalid Token {} {:?}", a,self.peek());
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
                    self.add_token(Token::Number(num));
                }
                c if c.unwrap().is_ascii_alphabetic() => {
                    let mut str = String::new();
                    str.push(c.unwrap());
                    while let Some(a)=self.next() {
                        if a.is_ascii_alphanumeric(){
                          str.push(a);
                        }
                    }
                    let key = self.keywords.get(&str);
                    if let Some(a) = key {
                        self.add_token(*a);
                    } else {
                        panic!("not an identifier {}", str);
                    }
                }
                Some('\n')=>{
                   self.line +=1;
                },
                Some(' ')  | Some('\t')=>{
                }
                a => panic!("{:?}",a),
            }
        }
        self.tokens.push(Token::Eof);
        &self.tokens
    }
    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        lexer.lexe().clone()
    }

    #[test]
    fn test_basic_math() {
        let tokens = tokenize("(1+2)");
        assert_eq!(
            tokens,
            vec![
                Token::LeftParen,
                Token::Number('1'),
                Token::Add,
                Token::Number('2'),
                Token::RightParen,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_logical_operators() {
        let tokens = tokenize("!(1>1+2)");
        assert_eq!(
            tokens,
            vec![
                Token::Not,
                Token::LeftParen,
                Token::Number('1'),
                Token::Greater,
                Token::Number('1'),
                Token::Add,
                Token::Number('2'),
                Token::RightParen,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_equality_and_inequality() {
        let tokens = tokenize("1==2!=3");
        assert_eq!(
            tokens,
            vec![
                Token::Number('1'),
                Token::EqualEquals,
                Token::Number('2'),
                Token::NotEqual,
                Token::Number('3'),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_greater_equal_and_lesser_equal() {
        let tokens = tokenize("1>=2<=3");
        assert_eq!(
            tokens,
            vec![
                Token::Number('1'),
                Token::GreaterEqual,
                Token::Number('2'),
                Token::LesserEqual,
                Token::Number('3'),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_nil_keyword() {
        let tokens = tokenize("nil");
        assert_eq!(tokens, vec![Token::Nil, Token::Eof]);
    }

    #[test]
    #[should_panic(expected = "not an identifier foo" )]
    fn test_invalid_identifier() {
        let _ = tokenize("foo");
    }

    #[test]
    #[should_panic(expected = "Invalid Token")]
    fn test_single_equal_panics() {
        let _ = tokenize("1=2");
    }

    #[test]
    fn test_multiple_lines() {
        let mut lexer = Lexer::new("1\n2\n3");
        let _ = lexer.lexe();
        assert_eq!(lexer.line, 2);
    }

    #[test]
fn test_less_than() {
    let tokens = tokenize("1<2");
    assert_eq!(
        tokens,
        vec![
            Token::Number('1'),
            Token::Lesser,
            Token::Number('2'),
            Token::Eof
        ]
    );
}

#[test]
fn test_less_than_equal() {
    let tokens = tokenize("1<=2");
    assert_eq!(
        tokens,
        vec![
            Token::Number('1'),
            Token::LesserEqual,
            Token::Number('2'),
            Token::Eof
        ]
    );
}

#[test]
fn test_mix_with_math() {
    let tokens = tokenize("(3+2)<(4*1)");
    assert_eq!(
        tokens,
        vec![
            Token::LeftParen,
            Token::Number('3'),
            Token::Add,
            Token::Number('2'),
            Token::RightParen,
            Token::Lesser,
            Token::LeftParen,
            Token::Number('4'),
            Token::Mul,
            Token::Number('1'),
            Token::RightParen,
            Token::Eof
        ]
    );
}

#[test]
fn test_less_than_at_end() {
    let tokens = tokenize("9<");
    assert_eq!(
        tokens,
        vec![
            Token::Number('9'),
            Token::Lesser,
            Token::Eof
        ]
    );
}

}

