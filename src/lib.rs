#[macro_use]
mod macros;

mod action;
mod error;
mod task;
mod utils;
mod vendor;

pub use error::UptError;
pub use utils::detect_os;
pub use vendor::{detect_vendor, init_vendor, Vendor};
