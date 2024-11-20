use inquire::Text;

use crate::{state::AppState, utils::ui::SafePrompt};

pub mod commands;

const PROMPT_SYMBOL: &str = "[>]";
const LOG_SYMBOL: &str = "[:]";

pub struct REPL<'a> {
    state: AppState<'a>,
}

impl<'s> REPL<'s> {
    pub fn new(state: AppState<'s>) -> Self {
        Self { state }
    }
    pub fn start(&self) {
        loop {
            let line = Text::new("[>] ").safely_prompt();
            if &line == "exit" {
                break;
            }
            println!("{line}");
        }
    }
}
