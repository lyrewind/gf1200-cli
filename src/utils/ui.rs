use inquire::{InquireError, Password, Text};

pub trait SafePrompt {
    fn safely_prompt(self) -> String;
}

/// Exits on <C-c> or <esc>.
const HANDLE_PROMPT_ERROR: fn(InquireError) -> String = |e| match e {
    InquireError::OperationInterrupted | InquireError::OperationCanceled => {
        println!("exiting...");
        std::process::exit(0);
    }
    _ => panic!("unexpected error prompting text."),
};

impl SafePrompt for Text<'_> {
    fn safely_prompt(self) -> String {
        self.prompt().unwrap_or_else(HANDLE_PROMPT_ERROR)
    }
}

impl SafePrompt for Password<'_> {
    fn safely_prompt(self) -> String {
        self.prompt().unwrap_or_else(HANDLE_PROMPT_ERROR)
    }
}
