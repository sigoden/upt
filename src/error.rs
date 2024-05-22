use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum UptError {
    NoVendor(String),
    NoTask,
    NoDetectVendor,
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
            NoTask => write!(f, "The package management tool cannot perform the task."),
            NoDetectVendor => write!(
                f,
                "No package management tool available, use `$UPT_TOOL` to specify one."
            ),
            InvalidAction(v) => write!(f, "Invalid action '{}'.", v),
            InvalidArgs(v) => write!(f, "Invalid arguments.\n\n{}", v),
            DisplayHelp(v) => write!(f, "{}", v),
        }
    }
}
