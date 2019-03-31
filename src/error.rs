
use std::fmt;
#[derive(Debug, PartialEq)]
pub enum UptError {
    NotFoundVender,
    InvalidArgs,
    NotRecongize,
}

impl fmt::Display for UptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
           UptError::NotFoundVender => write!(f, "Your os is not supported currently"),
           UptError::InvalidArgs => write!(f, "Invalid command line"),
           UptError::NotRecongize => write!(f, "Your command is invalid or not support, see help below"),
        }
    }
}