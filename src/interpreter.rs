use crate::{expre, stmt::Statement};

pub struct Interpreter{

}

impl Interpreter{
    pub fn new()->Interpreter{
        Self {  }
    }

    pub fn eval_statements(&mut self,statements:&[Statement])->Result<(),String>{
        for i in statements{
            match i {
                Statement::Print { expession }=>{
                    let expr = expession.expr()?;
                           match expr{
                            expre::Literal_Value::Nil => println!("nil"),
                            expre::Literal_Value::Number(a) => println!("{}", a),
                            expre::Literal_Value::String(a) => println!("{}", a.clone().borrow()),
                            expre::Literal_Value::Boolean(a) => println!("{}", a),
                        }

                },
                Statement::Expression { exp }=>{
                    exp.expr();
                }
            }
        }

        Ok(())
    }
}
