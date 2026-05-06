use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::app::eval::{string, State};
use crate::app::parser::{Expr, Stmt};
use crate::core::error::{label, line as error_line, value as error_value, Error};
use crate::core::help;
use crate::core::result::TypeResult;

pub fn eval(stmt: Stmt, state: &mut State) -> TypeResult<Option<String>> {
    match stmt {
        Stmt::All => all(),
        Stmt::Cd(expr) => cd(expr, state),
        Stmt::Clear => clear(),
        Stmt::Cwd => cwd(),
        Stmt::Dir(expr) => dir(expr, state),
        Stmt::Empty => Ok(None),
        Stmt::Exec(expr) => exec(expr, state),
        Stmt::Exit => exit(state),
        Stmt::Assign(name, expr) => assign(name, expr, state),
        Stmt::Print(name) => print(name, state),
        Stmt::Append(name, expr) => append(name, expr, state),
        Stmt::Pop(name, count) => pop(name, count, state),
        Stmt::Help(subject) => help(subject),
    }
}

fn assign(name: String, expr: Expr, state: &mut State) -> TypeResult<Option<String>> {
    let value = value(expr, state)?;
    state.vars.insert(name, value);
    Ok(None)
}

fn print(name: String, state: &State) -> TypeResult<Option<String>> {
    let value = get(&name, state)?;
    Ok(Some(string::display(value)))
}

fn append(name: String, expr: Expr, state: &mut State) -> TypeResult<Option<String>> {
    let suffix = value(expr, state)?;
    let target = get_mut(&name, state)?;
    target.push_str(&suffix);
    Ok(None)
}

fn pop(name: String, count: usize, state: &mut State) -> TypeResult<Option<String>> {
    let target = get_mut(&name, state)?;
    remove_tail(target, count);
    Ok(None)
}

fn help(subject: Option<String>) -> TypeResult<Option<String>> {
    match subject {
        Some(subject) => help_subject(&subject),
        None => Ok(Some(help::summary())),
    }
}

fn all() -> TypeResult<Option<String>> {
    Ok(Some(help::commands_summary()))
}

fn clear() -> TypeResult<Option<String>> {
    Ok(Some(_str::CLEAR_SCREEN.to_string()))
}

fn cwd() -> TypeResult<Option<String>> {
    let path = env::current_dir().map_err(|_| error_value(Error::CurrentDirRead))?;
    Ok(Some(path_to_string(path.as_path())))
}

fn cd(expr: Expr, state: &State) -> TypeResult<Option<String>> {
    let path = value(expr, state)?;
    env::set_current_dir(&path).map_err(|_| label(Error::DirChange, &path))?;
    Ok(None)
}

fn dir(expr: Expr, state: &State) -> TypeResult<Option<String>> {
    let path = value(expr, state)?;
    let mut items = Vec::new();
    let entries = fs::read_dir(&path).map_err(|_| label(Error::DirRead, &path))?;

    for entry in entries {
        let entry = entry.map_err(|_| label(Error::DirRead, &path))?;
        items.push(file_name(&entry.file_name()));
    }

    items.sort();
    Ok(Some(items.join(_str::LINE_BREAK)))
}

fn exec(expr: Expr, state: &State) -> TypeResult<Option<String>> {
    let command_line = value(expr, state)?;
    let parts = split_command(&command_line)?;
    let output = Command::new(&parts[0])
        .args(&parts[1..])
        .output()
        .map_err(|_| label(Error::ExecRun, &command_line))?;

    if !output.status.success() {
        let code = output.status.code().unwrap_or_default();
        return Err(error_line(Error::StatusCode, code as usize, &command_line));
    }

    Ok(Some(command_output(&output.stdout, &output.stderr)))
}

fn exit(state: &mut State) -> TypeResult<Option<String>> {
    state.exit = true;
    Ok(None)
}

fn help_subject(subject: &str) -> TypeResult<Option<String>> {
    help::get(subject)
        .map(Some)
        .ok_or_else(|| label(Error::UnknownHelpSubject, subject))
}

fn value(expr: Expr, state: &State) -> TypeResult<String> {
    match expr {
        Expr::Text(value) => Ok(value),
        Expr::Var(name) => get(&name, state).cloned(),
        Expr::Concat(parts) => concat(parts, state),
    }
}

fn concat(parts: Vec<Expr>, state: &State) -> TypeResult<String> {
    let mut result = String::new();

    for part in parts {
        result.push_str(&value(part, state)?);
    }

    Ok(result)
}

fn get<'a>(name: &str, state: &'a State) -> TypeResult<&'a String> {
    state
        .vars
        .get(name)
        .ok_or_else(|| label(Error::UndefinedVariable, name))
}

fn get_mut<'a>(name: &str, state: &'a mut State) -> TypeResult<&'a mut String> {
    state
        .vars
        .get_mut(name)
        .ok_or_else(|| label(Error::UndefinedVariable, name))
}

fn remove_tail(value: &mut String, count: usize) {
    for _ in 0..count {
        value.pop();
    }
}

fn split_command(line: &str) -> TypeResult<Vec<String>> {
    let parts = line
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<String>>();

    if parts.is_empty() {
        Err(error_value(Error::EmptyExec))
    } else {
        Ok(parts)
    }
}

fn command_output(stdout: &[u8], stderr: &[u8]) -> String {
    let out = String::from_utf8_lossy(stdout).trim().to_string();
    let err = String::from_utf8_lossy(stderr).trim().to_string();

    match (out.is_empty(), err.is_empty()) {
        (false, true) => out,
        (true, false) => err,
        (false, false) => format!("{}\n{}", out, err),
        (true, true) => String::new(),
    }
}

fn path_to_string(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

fn file_name(name: &std::ffi::OsStr) -> String {
    name.to_string_lossy().to_string()
}

mod _str {
    pub const CLEAR_SCREEN: &str = "\x1b[2J\x1b[H";
    pub const LINE_BREAK: &str = "\n";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assigns_and_prints_strings() {
        let mut state = State::default();

        eval(
            Stmt::Assign("a".to_string(), Expr::Text("hello".to_string())),
            &mut state,
        )
        .unwrap();

        let result = eval(Stmt::Print("a".to_string()), &mut state).unwrap();

        assert_eq!(result, Some("\"hello\"".to_string()));
    }

    #[test]
    fn appends_and_pops_from_existing_value() {
        let mut state = State::default();

        eval(
            Stmt::Assign("a".to_string(), Expr::Text("hello".to_string())),
            &mut state,
        )
        .unwrap();
        eval(
            Stmt::Append("a".to_string(), Expr::Text(" world".to_string())),
            &mut state,
        )
        .unwrap();
        eval(Stmt::Pop("a".to_string(), 3), &mut state).unwrap();

        assert_eq!(state.vars.get("a"), Some(&"hello wo".to_string()));
    }

    #[test]
    fn prints_all_commands() {
        let mut state = State::default();
        let result = eval(Stmt::All, &mut state).unwrap();

        assert!(result.unwrap().contains("dir(\"path\") - print all files and folders at \"path\""));
    }

    #[test]
    fn reads_current_working_directory() {
        let mut state = State::default();
        let result = eval(Stmt::Cwd, &mut state).unwrap();

        assert!(result.unwrap().contains("/"));
    }

    #[test]
    fn marks_state_for_exit() {
        let mut state = State::default();

        eval(Stmt::Exit, &mut state).unwrap();

        assert!(state.exit);
    }
}
