// src/main.rs
mod app;
mod core;

fn main() -> core::result::TypeResult<()> {
    let script = core::args::collect();

    if script.is_empty() {
        core::info::show();
        app::repl::start()?;
    } else {
        app::repl::run(&script)?;
    }

    Ok(())
}
