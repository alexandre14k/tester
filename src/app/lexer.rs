pub mod code;
pub mod string;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Text(String),
    Number(usize),
    Equal,
    Plus,
    Dot,
    LeftParen,
    RightParen,
    Comma,
    Eof,
}
