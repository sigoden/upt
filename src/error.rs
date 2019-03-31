
use std::fmt;
#[derive(Debug, PartialEq)]
pub enum UptError {
    NotFoundVendor(String),
    NotSupportOS,
    NotSupportTask,
    NoSubcommand,
    NotRecongize,
    BadOption(String),
}

impl fmt::Display for UptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
           UptError::NotFoundVendor(v) => write!(f, "No vendor {}", v),
           UptError::NoSubcommand => write!(f, "No subcommand"),
           UptError::NotSupportOS => write!(f, "Your os is not supported currently"),
           UptError::NotSupportTask =>  write!(f, "Task is not supported"),
           UptError::BadOption(v) => write!(f, "Invalid option {}", v),
           UptError::NotRecongize => write!(f, "Your input can not be recongized"),
        }
    }
}