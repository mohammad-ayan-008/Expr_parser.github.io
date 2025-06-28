use crate::expre::Expr;




pub enum Statement {
    // print "(" expr ")";
    Print{
        expession:Expr
    },
    Expression{
        exp:Expr
    }
}
