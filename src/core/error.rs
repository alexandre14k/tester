pub enum Error {
    CurrentDirRead,
    DirChange,
    DirRead,
    EmptyExec,
    ExecRun,
    ExpectedExpression,
    ExpectedIdentifier,
    ExpectedNumber,
    ExpectedStatement,
    ExpectedToken,
    ExpectedHelpSubject,
    InvalidNumber,
    ReservedLabel,
    StdinRead,
    StdoutFlush,
    ScriptRead,
    ScriptLine,
    StatusCode,
    UndefinedVariable,
    UnexpectedCharacter,
    UnexpectedInput,
    UnknownEscape,
    UnknownHelpSubject,
    UnknownMethod,
    UnterminatedEscape,
    UnterminatedString,
}

pub fn value(err: Error) -> String {
    match err {
        Error::CurrentDirRead => _str::CURRENT_DIR_READ.to_string(),
        Error::DirChange => _str::DIR_CHANGE.to_string(),
        Error::DirRead => _str::DIR_READ.to_string(),
        Error::EmptyExec => _str::EMPTY_EXEC.to_string(),
        Error::ExecRun => _str::EXEC_RUN.to_string(),
        Error::ExpectedExpression => _str::EXPECTED_EXPRESSION.to_string(),
        Error::ExpectedIdentifier => _str::EXPECTED_IDENTIFIER.to_string(),
        Error::ExpectedNumber => _str::EXPECTED_NUMBER.to_string(),
        Error::ExpectedStatement => _str::EXPECTED_STATEMENT.to_string(),
        Error::ExpectedToken => _str::EXPECTED_TOKEN.to_string(),
        Error::ExpectedHelpSubject => _str::EXPECTED_HELP_SUBJECT.to_string(),
        Error::InvalidNumber => _str::INVALID_NUMBER.to_string(),
        Error::ReservedLabel => {
            _str::RESERVED_LABEL.to_string()
        }
        Error::StdinRead => _str::STDIN_READ.to_string(),
        Error::StdoutFlush => _str::STDOUT_FLUSH.to_string(),
        Error::ScriptRead => _str::SCRIPT_READ.to_string(),
        Error::ScriptLine => _str::SCRIPT_LINE.to_string(),
        Error::StatusCode => _str::STATUS_CODE.to_string(),
        Error::UndefinedVariable => _str::UNDEFINED_VARIABLE.to_string(),
        Error::UnexpectedCharacter => _str::UNEXPECTED_CHARACTER.to_string(),
        Error::UnexpectedInput => _str::UNEXPECTED_INPUT.to_string(),
        Error::UnknownEscape => _str::UNKNOWN_ESCAPE.to_string(),
        Error::UnknownHelpSubject => _str::UNKNOWN_HELP_SUBJECT.to_string(),
        Error::UnknownMethod => _str::UNKNOWN_METHOD.to_string(),
        Error::UnterminatedEscape => _str::UNTERMINATED_ESCAPE.to_string(),
        Error::UnterminatedString => _str::UNTERMINATED_STRING.to_string(),
    }
}

pub fn label(err: Error, name: &str) -> String {
    _str::label(&value(err), name)
}

pub fn character(err: Error, ch: char) -> String {
    _str::character(&value(err), ch)
}

pub fn line(err: Error, number: usize, detail: &str) -> String {
    _str::line(&value(err), number, detail)
}

mod _str {
    pub const EXPECTED_EXPRESSION: &str = "expected expression";
    pub const CURRENT_DIR_READ: &str = "failed to read current working directory";
    pub const DIR_CHANGE: &str = "failed to change directory";
    pub const DIR_READ: &str = "failed to read directory";
    pub const EMPTY_EXEC: &str = "exec command is empty";
    pub const EXEC_RUN: &str = "failed to run command";
    pub const EXPECTED_IDENTIFIER: &str = "expected identifier";
    pub const EXPECTED_NUMBER: &str = "expected number";
    pub const EXPECTED_STATEMENT: &str = "expected statement";
    pub const EXPECTED_TOKEN: &str = "expected token";
    pub const EXPECTED_HELP_SUBJECT: &str = "expected help subject";
    pub const INVALID_NUMBER: &str = "invalid number";
    pub const RESERVED_LABEL: &str = "reserved label cannot be used as a variable or method target";
    pub const STDIN_READ: &str = "failed to read stdin";
    pub const STDOUT_FLUSH: &str = "failed to flush stdout";
    pub const SCRIPT_READ: &str = "failed to read script";
    pub const SCRIPT_LINE: &str = "error on line";
    pub const STATUS_CODE: &str = "command exited with status";
    pub const UNDEFINED_VARIABLE: &str = "undefined variable";
    pub const UNEXPECTED_CHARACTER: &str = "unexpected character";
    pub const UNEXPECTED_INPUT: &str = "unexpected input after statement";
    pub const UNKNOWN_ESCAPE: &str = "unknown escape";
    pub const UNKNOWN_HELP_SUBJECT: &str = "unknown help subject";
    pub const UNKNOWN_METHOD: &str = "unknown method";
    pub const UNTERMINATED_ESCAPE: &str = "unterminated escape";
    pub const UNTERMINATED_STRING: &str = "unterminated string";

    pub fn label(text: &str, name: &str) -> String {
        format!("{} `{}`", text, name)
    }

    pub fn character(text: &str, ch: char) -> String {
        format!("{} `{}`", text, ch)
    }

    pub fn line(text: &str, number: usize, detail: &str) -> String {
        format!("{} {}: {}", text, number, detail)
    }
}
