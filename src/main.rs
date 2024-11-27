use gf1200_cli::{api::Api, repl::{self, REPL}, state::AppState, utils::ui::SafePrompt};
use inquire::{Password, PasswordDisplayMode, Text};

fn main() {
    let (username, password) = prompt_login();
    let api = Api::new().authenticate(&username, &password);
    let commands = repl::commands::list_commands();
    let state = AppState { api, commands };

    println!("[#] entrando como '{username}'");
    REPL::new(state).start();
}

fn prompt_login() -> (String, String) {
    let username = Text::new("<usuário> ").safely_prompt();
    let password = Password::new("<senha> ")
        .with_display_toggle_enabled()
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_help_message("Ctrl+R pra exibir ou esconder.")
        .without_confirmation()
        .safely_prompt();
    (username, password)
}
