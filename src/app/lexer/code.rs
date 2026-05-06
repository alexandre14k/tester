use crate::app::lexer::{string, Token};
use crate::core::error::{character, value, Error};
use crate::core::result::TypeResult;

pub fn scan(input: &str) -> TypeResult<Vec<Token>> {
    let mut scanner = Scanner::new(input);
    scanner.scan()
}

struct Scanner {
    chars: Vec<char>,
    index: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            index: 0,
            tokens: Vec::new(),
        }
    }

    fn scan(&mut self) -> TypeResult<Vec<Token>> {
        while let Some(ch) = self.peek() {
            self.scan_char(ch)?;
        }

        self.tokens.push(Token::Eof);
        Ok(self.tokens.clone())
    }

    fn scan_char(&mut self, ch: char) -> TypeResult<()> {
        match ch {
            ' ' | '\t' | '\r' | '\n' => self.index += 1,
            '=' => self.push(Token::Equal),
            '+' => self.push(Token::Plus),
            '.' => self.push(Token::Dot),
            '(' => self.push(Token::LeftParen),
            ')' => self.push(Token::RightParen),
            ',' => self.push(Token::Comma),
            '"' => self.scan_text()?,
            ch if is_ident_start(ch) => self.scan_ident(),
            ch if ch.is_ascii_digit() => self.scan_number()?,
            _ => return Err(character(Error::UnexpectedCharacter, ch)),
        }

        Ok(())
    }

    fn push(&mut self, token: Token) {
        self.index += 1;
        self.tokens.push(token);
    }

    fn scan_text(&mut self) -> TypeResult<()> {
        let text = string::read(&self.chars, &mut self.index)?;
        self.tokens.push(Token::Text(text));
        Ok(())
    }

    fn scan_ident(&mut self) {
        let start = self.index;

        while self.peek().is_some_and(is_ident_part) {
            self.index += 1;
        }

        self.tokens.push(Token::Ident(self.slice(start)));
    }

    fn scan_number(&mut self) -> TypeResult<()> {
        let start = self.index;

        while self.peek().is_some_and(|ch| ch.is_ascii_digit()) {
            self.index += 1;
        }

        self.slice(start)
            .parse::<usize>()
            .map(Token::Number)
            .map(|token| self.tokens.push(token))
            .map_err(|_| value(Error::InvalidNumber))
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.index).copied()
    }

    fn slice(&self, start: usize) -> String {
        self.chars[start..self.index].iter().collect()
    }
}

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_ident_part(ch: char) -> bool {
    is_ident_start(ch) || ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_assignment_and_concat_tokens() {
        let tokens = scan("a=\"hi\"+b").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Ident("a".to_string()),
                Token::Equal,
                Token::Text("hi".to_string()),
                Token::Plus,
                Token::Ident("b".to_string()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn rejects_unexpected_characters() {
        let err = scan("@").unwrap_err();

        assert_eq!(err, "unexpected character `@`");
    }
}
