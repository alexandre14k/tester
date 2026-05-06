pub fn trim(line: &str) -> String {
    let mut quoted = false;
    let mut escaped = false;
    let mut value = String::new();

    for ch in line.chars() {
        if is_comment(ch, quoted, escaped) {
            break;
        }

        update_state(ch, &mut quoted, &mut escaped);
        value.push(ch);
    }

    value.trim().to_string()
}

fn is_comment(ch: char, quoted: bool, escaped: bool) -> bool {
    ch == '#' && !quoted && !escaped
}

fn update_state(ch: char, quoted: &mut bool, escaped: &mut bool) {
    if *escaped {
        *escaped = false;
    } else if ch == '\\' && *quoted {
        *escaped = true;
    } else if ch == '"' {
        *quoted = !*quoted;
    }
}
