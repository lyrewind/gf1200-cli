use inquire::Text;

use crate::{state::AppState, utils::ui::SafePrompt};

pub mod commands;

pub struct REPL<'s> {
    state: AppState<'s>,
}

impl<'s> REPL<'s> {
    pub const fn new(state: AppState<'s>) -> Self {
        Self { state }
    }
    pub fn start(&self) {
        loop {
            let line = Text::new("[>] ").safely_prompt();

            if &line == "exit" {
                break;
            }

            let mut chunks = line.split_whitespace();
            let Some(command) = chunks.next() else {
                continue;
            };
            let args: Vec<&str> = chunks.collect();

            let Some(command) = commands::find_command(command) else {
                println!("comando desconhecido: '{command}'.");
                continue;
            };

            if let Err(err) = command.validate_args(&args) {
                eprintln!("falha ao executar '{}'\n\\====> {err}", command.name);
            } else {
                (command.run)(&self.state, &args);
            }
        }
    }
}
