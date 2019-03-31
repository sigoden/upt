use std::fs;

#[macro_use]
mod macros;

mod parser;
pub mod vendor;
pub mod error;
pub mod task;

pub use error::UptError;
pub use vendor::Vendor;
pub use task::Task;

/// Detect os package management
pub fn detect_os_vendor() -> Result<Vendor, UptError> {
    if cfg!(target_os = "windows") {

    } else if cfg!(target_os = "macos") {

    } else if cfg!(target_os = "linux") {
        let release = fs::read_to_string("/etc/os-release").map_err(|_| UptError::NotSupportOS)?;
        let id = release
            .lines()
            .find(|l| l.starts_with("ID="))
            .ok_or_else(|| UptError::NotSupportOS)?;
        match &id[3..] {
            "arch" | "manjaro" => return Ok(vendor::pacman::init()),
            // "centos" | "redhat" => return Ok(vendor::yum::init()),
            // "fedora" => return Ok(vendor::dnf::init()),
            // "alpine" => return Ok(vendor::apk::init()),
            "debian" | "ubuntu" | "pop-os" | "deepin" | "elementary" => {
                return Ok(vendor::apt::init());
            }
            // "freebsd" => return Ok(vendor::pkg::init()),
            // "gentoo" => return Ok(vendor::emerge::init()),
            // "opensuse" => return Ok(vendor::zypper::init()),
            _ => {}
        }
    }
    Err(UptError::NotSupportOS)
}