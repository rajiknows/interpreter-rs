
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Ident(String),
    Int(i64),
    Infix(Box<Expr>, String, Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Expr(Expr),
    Print(Expr)
}
