---
title: Expr_parser
---

<style>
  body {
    background: #0d0d14;
    color: #e6e6e6;
    font-family: 'Segoe UI', sans-serif;
    padding: 24px;
  }

  h1, h2, h3 {
    color: #ff9edb;
  }

  a {
    color: #82aaff;
  }

  pre, code {
    background: #1a1b26;
    color: #e0def4;
    padding: 12px 16px;
    border-radius: 8px;
    font-family: 'Fira Code', monospace;
    font-size: 0.95em;
    line-height: 1.6;
    overflow-x: auto;
    display: block;
  }

  code span.keyword    { color: #ff79c6; }   /* pink keywords */
  code span.function   { color: #82aaff; }   /* soft blue functions */
  code span.variable   { color: #f8f8f2; }
  code span.comment    { color: #6272a4; font-style: italic; }
  code span.string     { color: #f1fa8c; }
  code span.number     { color: #bd93f9; }
</style>

<p align="left">
  <a href="https://freeimage.host/i/FTomCwQ">
    <img src="https://iili.io/FTomCwQ.jpg" 
         alt="FTomCwQ" 
         width="350" 
         style="border-radius: 12px; box-shadow: 0 4px 12px rgba(0,0,0,0.2);" />
  </a>
</p>


# A Simple Scanner 
We are creating a small Parser to parse simple  __*Mathematical  Expressions*__
just like 

<p align="left">
 <a href="https://freeimage.host/i/FTTeP3B">
    <img src="https://iili.io/FTTeP3B.md.jpg" 
         alt="FTomCwQ" 
         width="350" 
         style="border-radius: 12px; box-shadow: 0 4px 12px rgba(0,0,0,0.2);" />
  </a>
</p>
I'll Be writing all code in rust but you can write the same code in languages such as java or c . You can also use visitor pattern in java to represent the Grammar

# Let’s Begin
```rs
pub struct Lexer {}
```
A basic Representation of a lexer I know theres no need of something like an empty Struct but i just wrote it for fun . You can try to store the Source aka String inside a lexer
and then call related methods

**Tokens**
So Every little char we get as in input we will map it as a __TOKEN__

```rs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Number(char),
    Add,
    Sub,
    Div,
    Print,
    Mul,
    RightParen,
    LeftParen,
    Eof,
}
```

__LEXING__

<p align="left">
 <a href="https://freeimage.host/i/FTTlQa4">
    <img src="https://iili.io/FTTlQa4.md.jpg" 
         alt="FTomCwQ" 
         width="350" 
         style="border-radius: 12px; box-shadow: 0 4px 12px rgba(0,0,0,0.2);" />
  </a>
</p>

A Process to convert the Raw source code into tokens- small meaningful units like keywords , numbers ..  
```rs
impl Lexer {
    pub fn new() -> Self {
        Self {}
    }
    pub fn lexe(&mut self, source: &str) -> Vec<Token> {
        let mut tokens = source
            .chars() // Returns something like  Chars<'_> which is iterable or imagine it like "abc".chars() -> ['a','b','c']
            .filter(|x| !x.is_ascii_whitespace()) // skipping the white spaces 
            .map(|x| match x { // converting the string stream to Token stream (Something similar stream Api in java)
                '0'..='9' => Token::Number(x),
                '+' => Token::Add,
                '-' => Token::Sub,
                '*' => Token::Mul,
                '/' => Token::Div,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                ':' => Token::Print,
                _ => panic!("Bad token {}", x),
            })
            .collect::<Vec<Token>>();
        tokens.push(Token::Eof); // we manullay insert a EOF (END OF FILE TOKEN) which means that we completed the reading process of source 
        tokens
    }
}
```
