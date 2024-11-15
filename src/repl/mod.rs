use std::io::{self, BufRead, Write};

const PROMPT_SYMBOL: &str = "[>]";
const LOG_SYMBOL: &str = "[:]";

fn with_log(line: &str) -> String {
    format!("{LOG_SYMBOL} {line}")
}

pub struct REPL;

impl REPL {
    /// Starts running the REPL.
    pub fn start() {
        loop {
            let line = read_line();

            if &line == "exit" {
                break;
            } else {
                println!("{}", with_log(&line));
            }
        }
    }
}

/// Prompts and reads a line.
fn read_line() -> String {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut line = String::new();

    stdout
        .write_all(format!("{PROMPT_SYMBOL} ").as_bytes())
        .ok();
    stdout.flush().ok();
    stdin.read_line(&mut line).unwrap();

    line = line.trim().into();
    line
}
