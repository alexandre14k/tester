use crate::app::lexer::Token;
use crate::app::parser::{Expr, Stmt};
use crate::core::error::{label, value, Error};
use crate::core::result::TypeResult;

pub fn parse(tokens: &[Token]) -> TypeResult<Stmt> {
    Parser::new(tokens).parse()
}

struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, index: 0 }
    }

    fn parse(&mut self) -> TypeResult<Stmt> {
        match self.peek().clone() {
            Token::Eof => Ok(Stmt::Empty),
            Token::Ident(name) if self.is_command_call(&name) => self.parse_command(&name),
            Token::Ident(name) => self.parse_named(name),
            _ => Err(value(Error::ExpectedStatement)),
        }
    }

    fn parse_named(&mut self, name: String) -> TypeResult<Stmt> {
        self.expect_label(&name)?;
        self.advance();

        match self.peek() {
            Token::Equal => self.parse_assign(name),
            Token::Dot => self.parse_call(name),
            Token::Eof => Ok(Stmt::Print(name)),
            _ => Err(value(Error::ExpectedStatement)),
        }
    }

    fn parse_command(&mut self, name: &str) -> TypeResult<Stmt> {
        self.advance();
        self.expect(Token::LeftParen)?;

        match name {
            _str::ALL => self.finish_empty(Stmt::All),
            _str::CLEAR => self.finish_empty(Stmt::Clear),
            _str::CWD => self.finish_empty(Stmt::Cwd),
            _str::EXIT => self.finish_empty(Stmt::Exit),
            _str::HELP => self.parse_help_subject(),
            _str::CD => self.parse_single_expr(Stmt::Cd),
            _str::DIR => self.parse_single_expr(Stmt::Dir),
            _str::EXEC => self.parse_single_expr(Stmt::Exec),
            _ => Err(value(Error::ExpectedStatement)),
        }
    }

    fn parse_help_subject(&mut self) -> TypeResult<Stmt> {
        match self.peek() {
            Token::RightParen => self.finish_help(None),
            Token::Text(subject) => self.finish_help(Some(subject.clone())),
            _ => Err(value(Error::ExpectedHelpSubject)),
        }
    }

    fn finish_help(&mut self, subject: Option<String>) -> TypeResult<Stmt> {
        if subject.is_some() {
            self.advance();
        }

        self.expect(Token::RightParen)?;
        self.expect_eof()?;
        Ok(Stmt::Help(subject))
    }

    fn finish_empty(&mut self, stmt: Stmt) -> TypeResult<Stmt> {
        self.expect(Token::RightParen)?;
        self.expect_eof()?;
        Ok(stmt)
    }

    fn parse_single_expr<F>(&mut self, build: F) -> TypeResult<Stmt>
    where
        F: FnOnce(Expr) -> Stmt,
    {
        let expr = self.parse_expr()?;
        self.expect(Token::RightParen)?;
        self.expect_eof()?;
        Ok(build(expr))
    }

    fn parse_assign(&mut self, name: String) -> TypeResult<Stmt> {
        self.advance();
        let expr = self.parse_expr()?;
        self.expect_eof()?;
        Ok(Stmt::Assign(name, expr))
    }

    fn parse_call(&mut self, name: String) -> TypeResult<Stmt> {
        self.advance();
        let method = self.expect_ident()?;
        self.expect(Token::LeftParen)?;
        self.parse_method(name, method)
    }

    fn parse_method(&mut self, name: String, method: String) -> TypeResult<Stmt> {
        match method.as_str() {
            _str::APPEND => self.parse_append(name),
            _str::POP => self.parse_pop(name),
            _ => Err(label(Error::UnknownMethod, &method)),
        }
    }

    fn parse_append(&mut self, name: String) -> TypeResult<Stmt> {
        let expr = self.parse_expr()?;
        self.expect(Token::RightParen)?;
        self.expect_eof()?;
        Ok(Stmt::Append(name, expr))
    }

    fn parse_pop(&mut self, name: String) -> TypeResult<Stmt> {
        let count = self.expect_number()?;
        self.expect(Token::RightParen)?;
        self.expect_eof()?;
        Ok(Stmt::Pop(name, count))
    }

    fn parse_expr(&mut self) -> TypeResult<Expr> {
        let mut parts = vec![self.parse_term()?];

        while matches!(self.peek(), Token::Plus) {
            self.advance();
            parts.push(self.parse_term()?);
        }

        if parts.len() == 1 {
            Ok(parts.remove(0))
        } else {
            Ok(Expr::Concat(parts))
        }
    }

    fn parse_term(&mut self) -> TypeResult<Expr> {
        match self.peek() {
            Token::Text(value) => self.take_text(value.clone()),
            Token::Ident(name) => self.take_var(name.clone()),
            _ => Err(value(Error::ExpectedExpression)),
        }
    }

    fn take_text(&mut self, value: String) -> TypeResult<Expr> {
        self.advance();
        Ok(Expr::Text(value))
    }

    fn take_var(&mut self, name: String) -> TypeResult<Expr> {
        self.expect_label(&name)?;
        self.advance();
        Ok(Expr::Var(name))
    }

    fn expect(&mut self, token: Token) -> TypeResult<()> {
        if self.peek() == &token {
            self.advance();
            Ok(())
        } else {
            Err(value(Error::ExpectedToken))
        }
    }

    fn expect_eof(&self) -> TypeResult<()> {
        match self.peek() {
            Token::Eof => Ok(()),
            _ => Err(value(Error::UnexpectedInput)),
        }
    }

    fn expect_ident(&mut self) -> TypeResult<String> {
        match self.peek() {
            Token::Ident(name) => {
                let value = name.clone();
                self.advance();
                Ok(value)
            }
            _ => Err(value(Error::ExpectedIdentifier)),
        }
    }

    fn expect_number(&mut self) -> TypeResult<usize> {
        match self.peek() {
            Token::Number(value) => {
                let count = *value;
                self.advance();
                Ok(count)
            }
            _ => Err(value(Error::ExpectedNumber)),
        }
    }

    fn expect_label(&self, name: &str) -> TypeResult<()> {
        if is_reserved(name) {
            Err(label(Error::ReservedLabel, name))
        } else {
            Ok(())
        }
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.index).unwrap_or(&Token::Eof)
    }

    fn is_command_call(&self, name: &str) -> bool {
        is_reserved(name) && matches!(self.tokens.get(self.index + 1), Some(Token::LeftParen))
    }
}

fn is_reserved(name: &str) -> bool {
    matches!(
        name,
        _str::ALL | _str::CD | _str::CLEAR | _str::CWD | _str::DIR | _str::EXEC | _str::EXIT | _str::HELP | _str::QUIT
    )
}

mod _str {
    pub const ALL: &str = "all";
    pub const APPEND: &str = "append";
    pub const CD: &str = "cd";
    pub const CLEAR: &str = "clear";
    pub const CWD: &str = "cwd";
    pub const DIR: &str = "dir";
    pub const EXEC: &str = "exec";
    pub const EXIT: &str = "exit";
    pub const HELP: &str = "help";
    pub const POP: &str = "pop";
    pub const QUIT: &str = "quit";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_help_subject_call() {
        let stmt = parse(&[
            Token::Ident("help".to_string()),
            Token::LeftParen,
            Token::Text("syntax".to_string()),
            Token::RightParen,
            Token::Eof,
        ])
        .unwrap();

        assert_eq!(stmt, Stmt::Help(Some("syntax".to_string())));
    }

    #[test]
    fn rejects_reserved_assignment_labels() {
        let err = parse(&[
            Token::Ident("help".to_string()),
            Token::Equal,
            Token::Text("bad".to_string()),
            Token::Eof,
        ])
        .unwrap_err();

        assert_eq!(err, "reserved label cannot be used as a variable or method target `help`");
    }

    #[test]
    fn allows_method_names_as_variable_labels() {
        let stmt = parse(&[
            Token::Ident("append".to_string()),
            Token::Equal,
            Token::Text("value".to_string()),
            Token::Eof,
        ])
        .unwrap();

        assert_eq!(
            stmt,
            Stmt::Assign("append".to_string(), Expr::Text("value".to_string()))
        );
    }

    #[test]
    fn parses_command_with_string_argument() {
        let stmt = parse(&[
            Token::Ident("cd".to_string()),
            Token::LeftParen,
            Token::Text("/tmp".to_string()),
            Token::RightParen,
            Token::Eof,
        ])
        .unwrap();

        assert_eq!(stmt, Stmt::Cd(Expr::Text("/tmp".to_string())));
    }

    #[test]
    fn parses_empty_command_call() {
        let stmt = parse(&[
            Token::Ident("all".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::Eof,
        ])
        .unwrap();

        assert_eq!(stmt, Stmt::All);
    }

    #[test]
    fn parses_exit_command_call() {
        let stmt = parse(&[
            Token::Ident("exit".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::Eof,
        ])
        .unwrap();

        assert_eq!(stmt, Stmt::Exit);
    }
}
