use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum UptError {
    NoVendor(String),
    NoDetectVendor,
    InvalidTask,
    InvalidAction(String),
    InvalidArgs(String),
    DisplayHelp(String),
}

impl Error for UptError {}

impl fmt::Display for UptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UptError::*;
        match self {
            NoVendor(v) => write!(f, "The package management tool '{}' is not supported.", v),
            NoDetectVendor => write!(
                f,
                "No package management tool available, use `$UPT_TOOL` to specify one."
            ),
            InvalidTask => write!(f, "The package management tool cannot perform the task."),
            InvalidAction(v) => write!(f, "Invalid action '{}'.", v),
            InvalidArgs(v) => write!(f, "Invalid arguments.\n\n{}", v),
            DisplayHelp(v) => write!(f, "{}", v),
        }
    }
}
