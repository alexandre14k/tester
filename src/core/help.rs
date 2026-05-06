pub enum Help {
    Classes,
    Cmd,
    Functions,
    Help,
    Reserved,
    Rules,
    Syntax,
    Threads,
    Time,
    Types,
    Variables,
}

pub fn subjects() -> Vec<&'static str> {
    vec![
        _str::CLASSES,
        _str::CMD,
        _str::FUNCTIONS,
        _str::HELP,
        _str::RESERVED,
        _str::RULES,
        _str::SYNTAX,
        _str::THREADS,
        _str::TIME,
        _str::TYPES,
        _str::VARIABLES,
    ]
}

pub fn summary() -> String {
    let mut lines = Vec::new();
    lines.push(_str::HELP_SUMMARY.to_string());
    lines.push(_str::AVAILABLE_TOPICS.to_string());

    for subject in subjects() {
        lines.push(topic(subject));
    }

    lines.join(_str::LINE_BREAK)
}

fn topic(subject: &str) -> String {
    format!("{}{}", _str::TOPIC_PREFIX, subject)
}

pub fn commands() -> Vec<&'static str> {
    vec![
        _str::ALL_CMD,
        _str::CD_CMD,
        _str::CLEAR_CMD,
        _str::CWD_CMD,
        _str::DIR_CMD,
        _str::EXIT_CMD,
        _str::EXEC_CMD,
        _str::HELP_CMD,
    ]
}

pub fn commands_summary() -> String {
    commands()
        .into_iter()
        .map(command)
        .collect::<Vec<String>>()
        .join(_str::LINE_BREAK)
}

fn command(text: &str) -> String {
    format!("{}{}", _str::TOPIC_PREFIX, text)
}

pub fn get(subject: &str) -> Option<String> {
    match subject {
        _str::CLASSES => Some(value(Help::Classes)),
        _str::CMD => Some(value(Help::Cmd)),
        _str::FUNCTIONS => Some(value(Help::Functions)),
        _str::HELP => Some(value(Help::Help)),
        _str::RESERVED => Some(value(Help::Reserved)),
        _str::RULES => Some(value(Help::Rules)),
        _str::SYNTAX => Some(value(Help::Syntax)),
        _str::THREADS => Some(value(Help::Threads)),
        _str::TIME => Some(value(Help::Time)),
        _str::TYPES => Some(value(Help::Types)),
        _str::VARIABLES => Some(value(Help::Variables)),
        _ => None,
    }
}

fn value(help: Help) -> String {
    match help {
        Help::Classes => include_str!("help/classes.md").to_string(),
        Help::Cmd => include_str!("help/cmd.md").to_string(),
        Help::Functions => include_str!("help/functions.md").to_string(),
        Help::Help => include_str!("help/help.md").to_string(),
        Help::Reserved => include_str!("help/reserved.md").to_string(),
        Help::Rules => include_str!("help/rules.md").to_string(),
        Help::Syntax => include_str!("help/syntax.md").to_string(),
        Help::Threads => include_str!("help/threads.md").to_string(),
        Help::Time => include_str!("help/time.md").to_string(),
        Help::Types => include_str!("help/types.md").to_string(),
        Help::Variables => include_str!("help/variables.md").to_string(),
    }
}

mod _str {
    pub const CLASSES: &str = "classes";
    pub const CMD: &str = "cmd";
    pub const FUNCTIONS: &str = "functions";
    pub const HELP: &str = "help";
    pub const RESERVED: &str = "reserved";
    pub const RULES: &str = "rules";
    pub const SYNTAX: &str = "syntax";
    pub const THREADS: &str = "threads";
    pub const TIME: &str = "time";
    pub const TYPES: &str = "types";
    pub const VARIABLES: &str = "variables";
    pub const HELP_SUMMARY: &str = "User help(\"syntax\") to print syntax rules";
    pub const AVAILABLE_TOPICS: &str = "available topics:";
    pub const LINE_BREAK: &str = "\n";
    pub const TOPIC_PREFIX: &str = " - ";
    pub const ALL_CMD: &str = "all() - print all available commands";
    pub const CD_CMD: &str = "cd(\"path\") - change current working directory to \"path\"";
    pub const CLEAR_CMD: &str = "clear() - clear screen";
    pub const CWD_CMD: &str = "cwd() - print current working directory";
    pub const DIR_CMD: &str = "dir(\"path\") - print all files and folders at \"path\"";
    pub const EXIT_CMD: &str = "exit() - exit program";
    pub const EXEC_CMD: &str = "exec(\"line\") - run a system command from \"line\"";
    pub const HELP_CMD: &str = "help(\"topic\") - print help text for \"topic\"";
}
