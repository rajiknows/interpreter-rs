// ast stands for abstract syntax tree 
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Ident(String),// identifier
    Int(i64), 
    Infix(Box<Expr>, String, Box<Expr>),// infix expression 
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Expr(Expr),
    Print(Expr)
}
