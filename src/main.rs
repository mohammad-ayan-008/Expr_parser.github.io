use std::{
    io::{Write, stdin, stdout},
    vec,
};

use lexer::{Lexer, LiteralValue};
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
        let mut lexer = Lexer::new(&line);
        match lexer.lexe() {
            Ok(a) => {
                //  let tokens = lexer.lexe(line.as_str());
                let mut parser = Parser::new(lexer.tokens);
                match parser.expression().expr() {
                    Ok(a) => match a {
                        expre::Literal_Value::Nil => println!("nil"),
                        expre::Literal_Value::Number(a) => println!("{}", a),
                        expre::Literal_Value::String(a) => println!("{}", a.clone().borrow()),
                        expre::Literal_Value::Boolean(a) => println!("{}", a),
                    },
                    Err(e) => println!("{}", e),
                }
                line.clear();
            }
            Err(a) => println!("{}", a),
        }
    }
}
