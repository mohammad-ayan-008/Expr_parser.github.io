use std::{
    io::{Write, stdin, stdout},
    vec,
};

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
mod interpreter;
mod stmt;
mod expre;
mod lexer;
mod parser;
mod token;

fn main() {
    let mut line = String::new();

    loop {
        print!(">>");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();
        let mut interpreter = Interpreter::new();
        let mut lexer = Lexer::new(&line);
        match lexer.lexe() {
            Ok(a) => {

                let tokens = lexer.lexe();
                 let mut parser = Parser::new(lexer.tokens);
                 let stmt =parser.parse().unwrap();
                 interpreter.eval_statements(&stmt).unwrap();
                // match parser.statements() {
                //     Ok(e) => match e.expr() {
                //         Ok(liteal) => match liteal {
                //             expre::Literal_Value::Nil => println!("nil"),
                //             expre::Literal_Value::Number(a) => println!("{}", a),
                //             expre::Literal_Value::String(a) => println!("{}", a.clone().borrow()),
                //             expre::Literal_Value::Boolean(a) => println!("{}", a),
                //         },
                //         Err(a) => {
                //            line.clear();
                //            println!("{}",a);
                //         }
                //     },
                //     Err(a) => {
                //        line.clear();
                //        println!("{}",a);
                //     }
                // }
                line.clear();
            }
            Err(a) => {
                line.clear();
                println!("{}", a)
            }
        }
    }
}
