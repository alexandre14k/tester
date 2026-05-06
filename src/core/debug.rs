// src/core/macros.rs

// example : _inspect!("begin");
#[macro_export]
macro_rules! _inspect {
    ($val:expr) => {
        if $crate::core::init::FLAG_DEBUG {
            let content = $crate::core::debug::_str::value(&$val);
            let lines: Vec<&str> = content.lines().collect();
            let max_line_width = lines.len().to_string().len();

            $crate::core::debug::_str::print(
                $crate::core::debug::_str::head(
                    file!(),
                    line!(),
                    $crate::core::debug::_str::_DBG_ESP,
                    stringify!($val),
                ),
            );

            $crate::core::debug::_str::print(
                $crate::core::debug::_str::dot(
                    "",
                    $crate::core::debug::_str::_DBG_DOT,
                    max_line_width + 1,
                ),
            );

            for (i, line) in lines.iter().enumerate() {
                $crate::core::debug::_str::print(
                    $crate::core::debug::_str::line(
                        i + 1,
                        $crate::core::debug::_str::_DBG_LIN,
                        line,
                        max_line_width,
                    ),
                );
            }

            $crate::core::debug::_str::print(
                $crate::core::debug::_str::end(
                    "",
                    $crate::core::debug::_str::_DBG_EOL,
                    $crate::core::debug::_str::_DBG_NEL,
                    max_line_width + 1,
                ),
            );
        }
    };
}

pub mod _str {
    use std::fmt::Display;

    pub fn value<T: Display>(value: &T) -> String {
        format!("{}", value)
    }

    pub fn head(file: &str, line: u32, sep: &str, name: &str) -> String {
        format!("{}:{}{}{}", file, line, sep, name)
    }

    pub fn dot(prefix: &str, dot: &str, width: usize) -> String {
        format!("{:width$}{}", prefix, dot, width = width)
    }

    pub fn line(index: usize, marker: &str, text: &str, width: usize) -> String {
        format!("{:width$} {} {}", index, marker, text, width = width)
    }

    pub fn end(prefix: &str, end: &str, next: &str, width: usize) -> String {
        format!("{:width$}{}{}", prefix, end, next, width = width)
    }

    pub fn print(text: String) {
        println!("{}", text);
    }

    pub const _DBG_DOT: &str = ".";
    pub const _DBG_ESP: &str = " ";
    pub const _DBG_NEL: &str = "\n";
    pub const _DBG_LIN: &str = "│ ";
    pub const _DBG_EOL: &str = "└── EOF";
}
