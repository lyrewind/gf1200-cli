use crate::state::AppState;

pub struct Command {
    /// Name to call this command in the REPL.
    name: String,
    /// What this command does.
    description: String,
    /// A callback to execute when this command is called.
    run: Box<fn(&AppState)>,
}
