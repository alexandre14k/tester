use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::app::eval::{code, State};
use crate::app::lexer;
use crate::app::parser;
use crate::core::error::{line, value, Error};
use crate::core::result::TypeResult;

pub fn start() -> TypeResult<()> {
    let mut state = State::default();
    let stdin = io::stdin();

    loop {
        _str::print_inline(super::string::PROMPT);
        io::stdout()
            .flush()
            .map_err(|_| value(Error::StdoutFlush))?;

        let mut line = String::new();
        let size = stdin
            .read_line(&mut line)
            .map_err(|_| value(Error::StdinRead))?;

        if size == 0 || is_exit(&line) {
            break;
        }

        print_error(eval_line(&line, &mut state));

        if state.exit {
            break;
        }
    }

    Ok(())
}

pub fn run(script: &str) -> TypeResult<()> {
    let source = source(script)?;
    let mut state = State::default();

    crate::_inspect!(&source);

    if !confirm_run()? {
        return Ok(());
    }

    run_lines(&source, &mut state);
    Ok(())
}

fn source(script: &str) -> TypeResult<String> {
    if Path::new(script).is_file() {
        fs::read_to_string(script).map_err(|_| value(Error::ScriptRead))
    } else {
        Ok(script.to_string())
    }
}

fn confirm_run() -> TypeResult<bool> {
    _str::print_inline(_str::CONFIRM_RUN);
    io::stdout()
        .flush()
        .map_err(|_| value(Error::StdoutFlush))?;

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .map_err(|_| value(Error::StdinRead))?;

    Ok(is_confirmed(&answer))
}

fn is_confirmed(answer: &str) -> bool {
    matches!(answer.trim(), _str::YES_LOWER | _str::YES_UPPER)
}

fn run_lines(source: &str, state: &mut State) {
    for (index, line) in source.lines().enumerate() {
        if let Err(err) = eval_line(line, state) {
            print_script_error(index + 1, line, &err);
            break;
        }

        if state.exit {
            break;
        }
    }
}

fn print_script_error(line_number: usize, source_line: &str, err: &str) {
    _str::print(line(Error::ScriptLine, line_number, err));

    if !source_line.trim().is_empty() {
        _str::print(source_line.to_string());
    }
}

fn print_error(result: TypeResult<()>) {
    match result {
        Ok(()) => {}
        Err(err) => _str::print(_str::error(&err)),
    }
}

fn eval_line(line: &str, state: &mut State) -> TypeResult<()> {
    match eval_text(line, state)? {
        Some(value) => _str::print(value),
        None => {}
    }

    Ok(())
}

fn eval_text(line: &str, state: &mut State) -> TypeResult<Option<String>> {
    let text = parser::string::trim(line);
    let tokens = lexer::code::scan(&text)?;
    let stmt = parser::code::parse(&tokens)?;
    code::eval(stmt, state)
}

fn is_exit(line: &str) -> bool {
    matches!(line.trim(), _str::EXIT | _str::EXIT_CALL | _str::QUIT)
}

mod _str {
    pub const CONFIRM_RUN: &str = "run script? [y/N] ";
    pub const YES_LOWER: &str = "y";
    pub const YES_UPPER: &str = "Y";
    pub const EXIT: &str = "exit";
    pub const EXIT_CALL: &str = "exit()";
    pub const QUIT: &str = "quit";

    pub fn error(err: &str) -> String {
        format!("error: {}", err)
    }

    pub fn print(text: String) {
        println!("{}", text);
    }

    pub fn print_inline(text: &str) {
        print!("{}", text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confirms_only_explicit_yes_values() {
        assert!(is_confirmed("y"));
        assert!(is_confirmed("Y\n"));
        assert!(!is_confirmed("n"));
        assert!(!is_confirmed(""));
    }

    #[test]
    fn detects_exit_commands() {
        assert!(is_exit("exit"));
        assert!(is_exit("exit()"));
        assert!(is_exit("quit\n"));
        assert!(!is_exit("help()"));
    }
}
