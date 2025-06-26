use std::io::{Write, stdin, stdout};

use lexer::Lexer;
use parser::Parser;

mod expre;
mod lexer;
mod parser;

fn main() {
    let mut line = String::new();

    loop {
        print!(">>");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();
        let  mut lexer = Lexer::new(&line);

        lexer.lexe();
        println!("{:?}",lexer.tokens);
        //  let tokens = lexer.lexe(line.as_str());
        let mut parser = Parser::new(lexer.tokens);
        println!("{:?}",parser.expression().expr());
        line.clear();
    }
}
