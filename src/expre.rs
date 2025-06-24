use crate::lexer::Token;

#[derive(Debug)]
pub enum  Expr {
    Number(f64),
    Group(Box<Expr>),
    Binary{
        left:Box<Expr>,
        op:Token,
        right:Box<Expr>
    },
    
}



impl Expr{
    pub fn expr(&self)->f64{
        match self {
            Expr::Number(x)=> *x,
            Expr::Group(x)=> x.expr(),
            Expr::Binary { left, op, right }=>{
                let val =left.expr();
                let val2 = right.expr();

                match  op{
                    Token::Sub=>{
                        (val- val2)
                    },
                    Token::Add =>{
                        (val + val2)
                    },
                    Token::Div=>{
                        (val / val2)
                    },
                    Token::Mul=>{
                        (val * val2)
                    }
                    _=>todo!()
                }
            }
        }
    }
}



