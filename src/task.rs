/// General tasks that every vender provides
#[derive(Debug, PartialEq)]
pub enum Task {
    /// install packages
    Install { pkg: String, assume_yes: bool },
    /// remove packages
    Remove { pkg: String, assume_yes: bool },
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
