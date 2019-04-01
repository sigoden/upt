use std::error::Error;
use std::fmt;
#[derive(Debug, PartialEq)]
pub enum UptError {
    NoVendor(String),
    NotSupportOS,
    NotSupportTask,
    NoSubcommand,
    NotRecongize,
    BadOption(String),
}

impl Error for UptError {}

impl fmt::Display for UptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UptError::*;
        match self {
            NoVendor(v) => write!(f, "No vendor for {}", v),
            NoSubcommand => write!(f, "No subcommand"),
            NotSupportOS => write!(f, "Your os is not supported currently"),
            NotSupportTask => write!(f, "Task is not supported"),
            BadOption(v) => write!(f, "Invalid option {}", v),
            NotRecongize => write!(f, "Your input can not be recongized"),
        }
    }
}
