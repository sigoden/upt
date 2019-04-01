use std::error::Error;
use std::fmt;
#[derive(Debug, PartialEq)]
pub enum UptError {
    NoVendor(String),
    NotSupportOS,
    NotSupportTask,
    NotRecongize,
    BadOption(String),
}

impl Error for UptError {}

impl fmt::Display for UptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UptError::*;
        match self {
            NoVendor(v) => write!(f, "Vendor {} is not supported", v),
            NotSupportOS => write!(f, "Your os is not supported currently"),
            NotSupportTask => write!(f, "Task is not supported by your os"),
            BadOption(v) => write!(f, "Option `{}` is invalid", v),
            NotRecongize => write!(f, "Your input can not be recongized"),
        }
    }
}
