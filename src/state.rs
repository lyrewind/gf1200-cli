use crate::{api::{Api, Authenticated}, repl::commands::Command};

pub struct AppState<'a> {
    pub api: Api<'a, Authenticated>,
    pub commands: Vec<Command>
}
