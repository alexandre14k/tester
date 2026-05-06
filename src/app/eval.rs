use std::collections::HashMap;

pub mod code;
pub mod string;

#[derive(Default)]
pub struct State {
    pub exit: bool,
    pub vars: HashMap<String, String>,
}
