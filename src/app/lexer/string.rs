use crate::core::error::{character, value, Error};
use crate::core::result::TypeResult;

pub fn read(chars: &[char], index: &mut usize) -> TypeResult<String> {
    let mut text = String::new();
    *index += 1;

    while *index < chars.len() {
        let ch = chars[*index];
        *index += 1;

        if ch == '"' {
            return Ok(text);
        }

        if ch == '\\' {
            text.push(read_escape(chars, index)?);
        } else {
            text.push(ch);
        }
    }

    Err(value(Error::UnterminatedString))
}

fn read_escape(chars: &[char], index: &mut usize) -> TypeResult<char> {
    let ch = chars
        .get(*index)
        .copied()
        .ok_or_else(|| value(Error::UnterminatedEscape))?;

    *index += 1;

    match ch {
        '"' => Ok('"'),
        '\\' => Ok('\\'),
        'n' => Ok('\n'),
        'r' => Ok('\r'),
        't' => Ok('\t'),
        _ => Err(character(Error::UnknownEscape, ch)),
    }
}
