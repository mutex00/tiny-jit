
/* ================= AST ================= */

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign(String, Expr),
    Print(Expr),
    Return(Expr),
    FunctionDef(Function),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}