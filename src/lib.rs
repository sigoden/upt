use std::fs;

#[macro_use]
mod macros;

mod parser;
pub mod vendor;
pub mod error;

pub use error::UptError;
pub use vendor::Vendor;

/// General tasks that every vender provides
pub enum Task {
    /// install packages
    Install { pkg: String, assume_yes: bool },
    /// uninstall packages
    Uninstall { pkg: String, assume_yes: bool },
    /// upgrade packages
    Upgrade { pkg: String, assume_yes: bool },
    /// search for a package
    Search { pkg: String },
    /// show details about a package
    Show { pkg: String },
    /// sync packages index
    UpdateIndex,
    /// upgrade all outdated packages
    UpgradeAll,
    /// list all upgradable packages
    ListUpgradable,
    /// list all installed packages
    ListInstalled,
}

/// Lookup vender by name
pub fn lookup_vendor(key: &str) -> Result<Vendor, UptError> {
    match key {
        // "upt" => return Ok(crate::vendor::upt::init),
        "apt" => return Ok(crate::vendor::apt::init()),
        _ => {},
    }
    Err(UptError::NotFoundVendor(key.to_string()))
}

/// Detect os package management
pub fn detect_os_vendor() -> Result<Vendor, UptError> {
    if cfg!(target_os = "windows") {

    } else if cfg!(target_os = "macos") {

    } else if cfg!(target_os = "linux") {
        let release = fs::read_to_string("/etc/os-release").map_err(|_| UptError::NotSupportOS)?;
        let id = release.lines().find(|l| l.starts_with("ID=")).ok_or_else(|| UptError::NotSupportOS)?;
        match &id[3..] {
            // "arch" | "manjaro" => return Ok(crate::vendor::pacman::init()),
            // "centos" | "redhat" => return Ok(crate::vendor::yum::Yum),
            // "fedora" => return Ok(crate::vendor::dnf::Dnf),
            // "alpine" => return Ok(crate::vendor::apk::Apk),
            "debian" | "ubuntu" | "pop-os" | "deepin" | "elementary" => return Ok(crate::vendor::apt::init()),
            // "freebsd" => return Ok(crate::vendor::pkg::Pkg),
            // "gentoo" => return Ok(crate::vendor::emerge::Emerge),
            // "opensuse" => return Ok(crate::vendor::zypper::Zypper),
            _ => {},
        }
    }
    Err(UptError::NotSupportOS)
}

