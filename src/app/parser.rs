pub mod code;
pub mod string;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Text(String),
    Var(String),
    Concat(Vec<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Empty,
    Assign(String, Expr),
    Print(String),
    Append(String, Expr),
    Pop(String, usize),
    Help(Option<String>),
    Clear,
    Cwd,
    Cd(Expr),
    Dir(Expr),
    Exec(Expr),
    Exit,
    All,
}
