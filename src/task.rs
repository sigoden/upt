/// General tasks that every vender provides
#[derive(Debug, PartialEq)]
pub enum Task {
    /// install packages
    Install { pkg: String, confirm: bool },
    /// remove packages
    Remove { pkg: String, confirm: bool },
    /// upgrade packages
    Upgrade { pkg: String, confirm: bool },
    /// search for a package
    Search { pkg: String },
    /// show a package info
    Info { pkg: String },
    /// sync packages index
    UpdateIndex,
    /// upgrade all outdated packages
    UpgradeAll { confirm: bool },
    /// list all upgradable packages
    ListUpgradable,
    /// list all installed packages
    ListInstalled,
}
