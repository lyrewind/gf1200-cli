use std::{
    fmt::{Debug, Display},
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use thiserror::Error;

use crate::state::AppState;

pub mod generic;
pub mod lan;
pub mod repl;

#[allow(clippy::wildcard_imports)]
/// Gets a list of all registered commands.
pub fn list_commands() -> Vec<Command> {
    use generic::*;
    use lan;
    use repl;

    Vec::from([devices(), device(), restart(), lan::status(), repl::help()])
}

#[derive(Clone)]
pub struct Command {
    /// A name to call this command in the REPL.
    pub name: &'static str,
    /// What this command does.
    pub description: &'static str,
    /// A list of arguments this command takes, if any.
    pub args: Option<Vec<Arg>>,
    /// A callback to execute when this command is called.
    pub run: fn(&AppState, &Vec<&str>),
}

impl Command {
    /// Checks for missing or extra arguments, and then typing.
    /// Called before the `run` callback.
    pub fn validate_args(&self, given_args: &Vec<&str>) -> Result<(), ArgValidationError> {
        if let Some(args) = &self.args {
            if given_args.len() > args.len() {
                Err(ArgValidationError::ExtraArgs(given_args.len() - args.len()))
            } else if given_args.len() < args.len() {
                let missing: Vec<Arg> = args.iter().skip(given_args.len()).cloned().collect();
                Err(ArgValidationError::Missing(missing))
            } else {
                if let Some(type_mismatch) = given_args
                    .iter()
                    .zip(args)
                    .find(|(given_arg, arg)| !arg.typing.matches(given_arg))
                {
                    Err(ArgValidationError::WrongType {
                        arg: type_mismatch.1.clone(),
                        found: ArgType::from_value(type_mismatch.0),
                    })
                } else {
                    Ok(())
                }
            }
        } else {
            if given_args.is_empty() {
                Ok(())
            } else {
                Err(ArgValidationError::CommandTakesNoArgs)
            }
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arg_line = match &self.args {
            Some(args) => args
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" "),
            None => "".to_string(),
        };

        write!(
            f,
            "{}{}{arg_line} - {}",
            self.name,
            if !arg_line.is_empty() { " " } else { &arg_line },
            self.description
        )
    }
}

#[derive(Error, Debug)]
pub enum ArgValidationError {
    #[error("esse comando não leva nenhum argumento.")]
    CommandTakesNoArgs,
    #[error("esse comando leva {0} argumentos a menos.")]
    ExtraArgs(usize),
    #[error("argumentos necessários: {0:?}")]
    Missing(Vec<Arg>),
    #[error("esperado argumento: {arg}, mas foi recebido {found}.")]
    WrongType { arg: Arg, found: ArgType },
}

#[derive(Clone, Debug)]
pub struct Arg {
    name: &'static str,
    typing: ArgType,
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.typing)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ArgType {
    String,
    Bool,
    IpAddress,
}

impl Debug for ArgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::String => "qualquer string",
            Self::Bool => "um de: '0', '1', 'true' ou 'false'",
            Self::IpAddress => "um endereço IPv4 ou IPv6 válido",
        };

        write!(f, "{text}")
    }
}

impl Display for ArgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::String => "string",
            Self::Bool => "boolean",
            Self::IpAddress => "endereço IPv4/6",
        };

        write!(f, "{text}")
    }
}

impl ArgType {
    /// Checks for what type a `value` would be.
    pub fn from_value(value: &str) -> Self {
        match value {
            "0" | "1" | "true" | "false" => Self::Bool,
            other if Ipv4Addr::from_str(other).is_ok() || Ipv6Addr::from_str(other).is_ok() => {
                Self::IpAddress
            }
            _ => Self::String,
        }
    }
    /// Checks if `value` matches this type.
    pub fn matches(&self, value: &str) -> bool {
        match self {
            Self::Bool => matches!(value, "0" | "1" | "true" | "false"),
            Self::IpAddress => {
                Ipv4Addr::from_str(value).is_ok() || Ipv6Addr::from_str(value).is_ok()
            }
            Self::String => true,
        }
    }
}

pub fn find_command(name: &str) -> Option<Command> {
    list_commands().iter().find(|cmd| cmd.name == name).cloned()
}
