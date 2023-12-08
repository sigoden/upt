use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum UptError {
    NoVendor(String),
    NotSupportOS,
    NotSupportTask,
    NotFoundTool,
    InvalidArgs(String),
}

impl Error for UptError {}

impl fmt::Display for UptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UptError::*;
        match self {
            NoVendor(v) => write!(f, "Vendor {} is not supported", v),
            NotSupportOS => write!(f, "Your OS is not supported currently"),
            NotSupportTask => write!(f, "The task is not supported by your os"),
            NotFoundTool => write!(f, "No found package management tool"),
            InvalidArgs(v) => write!(f, "Invalid arguments\n{}", v),
        }
    }
}
