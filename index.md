---
title: Expression Parser
---
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<style>
  :root {
    --bg: #1a1a2e;
    --fg: #f8e9f0;
    --comment: #7e6593;
    --string: #ffb1c1;
    --keyword: #ff79c6;
    --function: #f8f8f2;
    --number: #bd93f9;
    --type: #8be9fd;
  }
  body {
    background: var(--bg);
    color: var(--fg);
    font-family: 'Segoe UI', sans-serif;
    padding: 20px;
  }

  h1, h2, h3 {
    color: #ff9edb;
  }

  a {
    color: #82aaff;
  }
  
  pre, .highlight {
    background: #252537 !important;
    color: var(--fg) !important;
    padding: 1rem;
    border-radius: 6px;
    overflow-x: auto;
    font-family: 'Fira Code', monospace;
    font-size: 0.95em;
  }

  pre code {
    background: none !important;
    color: inherit !important;
  }

  /* Syntax Highlighting */
  .highlight .c,
  .highlight .cm {
    color: var(--comment);
    font-style: italic;
  }

  .highlight .s,
  .highlight .sb {
    color: var(--string);
  }

  .highlight .k {
    color: var(--keyword);
  }

  .highlight .nf {
    color: var(--function);
  }

  .highlight .mi,
  .highlight .m {
    color: var(--number);
  }

  .highlight .nc,
  .highlight .nt {
    color: var(--type);
  }

  img {
    max-width: 100%;
    border-radius: 6px;
    box-shadow: 0 0 10px #000;
  }
/* Make punctuation and operators white */
/* Only override punctuation and operators */
.highlight .p,
.highlight .o,
pre code span:not([class]) {
  color: var(--fg) !important;
}
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
<a href="https://freeimage.host/i/FTbvXFR"><img src="https://iili.io/FTbvXFR.md.jpg" alt="FTbvXFR.md.jpg" border="0"></a>
<p align="left">
 <a href="https://freeimage.host/i/FTbvXFR">
    <img src="https://iili.io/FTbvXFR.md.jpg" 
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

<p align="left">
  <a href="https://freeimage.host/i/FTb02Bn">
    <img src="https://iili.io/FTb02Bn.md.jpg" 
         alt="FTomCwQ" 
         width="350" 
         style="border-radius: 12px; box-shadow: 0 4px 12px rgba(0,0,0,0.2);" />
  </a>
</p>

