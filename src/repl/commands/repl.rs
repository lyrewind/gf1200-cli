use super::Command;

pub fn help() -> Command {
    Command {
        name: "help",
        description: "lista todos os comandos.",
        args: None,
        run: |state, _| {
            println!("comandos disponíveis:");
            for command in &state.commands {
                println!("[>] {command}");
            }
        },
    }
}
