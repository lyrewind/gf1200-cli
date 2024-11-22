use std::{
    fmt::{Debug, Display}, net::{Ipv4Addr, Ipv6Addr}, str::FromStr
};

use thiserror::Error;

use crate::state::AppState;

pub mod generic;

#[allow(clippy::wildcard_imports)]
/// Gets a list of all registered commands.
pub fn list_commands() -> Vec<Command> {
    use generic::*;

    Vec::from([devices(), device()])
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

#[derive(Error, Debug)]
pub enum ArgValidationError {
    #[error("this command takes no argument.")]
    CommandTakesNoArgs,
    #[error("this command takes {0} fewer arguments than provided.")]
    ExtraArgs(usize),
    #[error("missing arguments: {0:?}")]
    Missing(Vec<Arg>),
    #[error("expected argument: {arg}, found {found}.")]
    WrongType { arg: Arg, found: ArgType },
}

#[derive(Clone, Debug)]
pub struct Arg {
    name: &'static str,
    typing: ArgType,
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' ({})", self.name, self.typing)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ArgType {
    String,
    Bool,
    IpAddress,
}

impl Display for ArgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::String => "any string.",
            Self::Bool => "one of: '0', '1', 'true' or 'false'.",
            Self::IpAddress => "a valid IPv4 or IPv6 address.",
        };

        write!(f, "{text}")
    }
}

impl Debug for ArgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::String => "string",
            Self::Bool => "boolean",
            Self::IpAddress => "IPv4 or IPv6 address",
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
