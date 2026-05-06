// src/core/args.rs

pub fn collect() -> String {
    std::env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .join(_str::ARG_SEP)
}

mod _str {
    pub const ARG_SEP: &str = " ";
}
