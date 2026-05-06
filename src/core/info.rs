// src/core/info.rs

pub fn show() {
    let text = include_str!("info.md");
    _str::print(text);
}

mod _str {
    pub fn print(text: &str) {
        println!("{}", text);
    }
}
