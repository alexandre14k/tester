# tester

`tester` is a REPL (read-eval-print-loop) written in Rust.

It is structured for ease of understanding, incremental evolution, and direct reading of the codebase. The current implementation focuses on string features and a small set of system features. It is intended as an educational interpreter project, not as a replacement for Python.

![Visitors](https://api.visitorbadge.io/api/VisitorHit?user=alexandre14k&repo=https://github.com/alexandre14k/tester&label=Views&labelColor=%23555555&countColor=%23007EC6)

## Status

The project is open to continued evolution.

Joining requests may be sent to:

`alexander14k28@gmail.com`


## Principles

- Clear module boundaries
- Readable control flow
- Small functions
- Localized string definitions
- Translation-friendly user-facing text
- Educational value before feature volume

## Current Features

- Interactive REPL loop
- Script execution with confirmation
- String assignment and concatenation
- String methods:
  - `append(...)`
  - `pop(...)`
- Help topics loaded at compile time
- Built-in commands:
  - `all()`
  - `clear()`
  - `cwd()`
  - `cd("path")`
  - `dir("path")`
  - `exec("line")`
  - `help()`
  - `help("topic")`
  - `exit()`

## Project Intent

This project is free to copy, free to use, free to evolve, and free to adapt through translation of user-facing strings.

The code is generic by design. Its purpose is to help readers understand interpreter structure where existing books, examples, or explanations may feel incomplete.

## Joining

The project is open to contributors who want to help evolve it in a clear and disciplined way.

Requests to join may be sent to:

`alexander14k28@gmail.com`

## Build Notes

- The project has no external crate dependencies.
- Rust and Cargo are used for building and testing.
- A static musl target is used for the release build flow in `make.sh`.
- GCC is used only as part of the build toolchain and is not integrated into the project source itself.
- Current release output example: `out/tester` at `623.023 KB`
- Current release output is statically linked
- Current support target is Linux amd64/x86_64
- Broader platform support can evolve with contributor work

## License

This project uses the MIT License.

See [LICENSE](LICENSE).

## Acknowledgement

Honorable mention to Robert Nystrom for sharing the secret of REPL design:

<https://craftinginterpreters.com/the-lox-language.html>
