use std::io::{Write, stdin, stdout};

use lexer::Lexer;
use parser::Parser;

mod expre;
mod lexer;
mod parser;

fn main() {
    let mut line = String::new();
    let mut lexer = Lexer::new();

    loop {
        print!(">>");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();
        let tokens = lexer.lexe(line.as_str());
        let mut parser = Parser::new(tokens);
        println!("{}",parser.parse_expr().expr());
        line.clear();
    }
}
