#[macro_use]
mod macros;

mod action;
mod error;
mod task;
mod utils;
mod vendor;

pub use error::UptError;
pub use vendor::{detect_tool, init as init_vendor, Vendor};
