use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum UptError {
    NoVendor(String),
    NoTask,
    NotFoundTool,
    InvalidAction(String),
    InvalidArgs(String),
    DisplyHelp(String),
}

impl Error for UptError {}

impl fmt::Display for UptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UptError::*;
        match self {
            NoVendor(v) => write!(f, "The vendor {} is not supported.", v),
            NoTask => write!(f, "The package management tool cannot perform the task."),
            NotFoundTool => write!(
                f,
                "No found package management tool, use `$UPT_TOOL` to specify one."
            ),
            InvalidAction(v) => write!(f, "Invalid action '{}'.", v),
            InvalidArgs(v) => write!(f, "Invalid arguments.\n\n{}", v),
            DisplyHelp(v) => write!(f, "{}", v),
        }
    }
}
