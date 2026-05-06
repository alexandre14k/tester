pub fn display(value: &str) -> String {
    _str::display(&escape(value))
}

fn escape(value: &str) -> String {
    let mut result = String::new();

    for ch in value.chars() {
        push_char(&mut result, ch);
    }

    result
}

fn push_char(result: &mut String, ch: char) {
    match ch {
        '"' => result.push_str(_str::ESCAPED_QUOTE),
        '\\' => result.push_str(_str::ESCAPED_SLASH),
        '\n' => result.push_str(_str::ESCAPED_NEWLINE),
        '\r' => result.push_str(_str::ESCAPED_RETURN),
        '\t' => result.push_str(_str::ESCAPED_TAB),
        _ => result.push(ch),
    }
}

mod _str {
    pub const ESCAPED_QUOTE: &str = "\\\"";
    pub const ESCAPED_SLASH: &str = "\\\\";
    pub const ESCAPED_NEWLINE: &str = "\\n";
    pub const ESCAPED_RETURN: &str = "\\r";
    pub const ESCAPED_TAB: &str = "\\t";

    pub fn display(value: &str) -> String {
        format!("\"{}\"", value)
    }
}
