use std::fs;
use std::process::{Command, Stdio};

use crate::error::UptError;
use crate::subcommand::SubCommand;
use crate::task::Task;

mod apk;
mod apt;
mod brew;
mod choco;
mod dnf;
mod pacman;
mod scoop;
mod upt;
mod yum;

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";
/// Repersent a kind of package management tool. e.g. apt, pacman, yum...
#[derive(Debug)]
pub struct Vendor {
    pub(crate) name: String,
    pub(crate) yes: Vec<String>,
    pub(crate) install: SubCommand,
    pub(crate) remove: SubCommand,
    pub(crate) upgrade: SubCommand,
    pub(crate) search: SubCommand,
    pub(crate) show: SubCommand,
    pub(crate) update_index: SubCommand,
    pub(crate) upgrade_all: SubCommand,
    pub(crate) list_upgradable: SubCommand,
    pub(crate) list_installed: SubCommand,
}

/// A kind of package management tool. e.g. apt, pacman, yum...
impl Vendor {
    /// Lookup vender by name
    pub fn lookup(name: &str) -> Result<Vendor, UptError> {
        let vendor = match name {
            "apk" => apk::init(),
            "apt" => apt::init(),
            "brew" => brew::init(),
            "choco" => choco::init(),
            "scoop" => scoop::init(),
            "dnf" => dnf::init(),
            "pacman" => pacman::init(),
            "upt" => upt::init(),
            "yum" => yum::init(),
            _ => return Err(UptError::NoVendor(name.to_string())),
        };
        Ok(vendor)
    }
    /// Detect package management on os
    pub fn detect() -> Result<Vendor, UptError> {
        if cfg!(target_os = "windows") {
            if which("scoop") {
                return Ok(scoop::init());
            }
            return Ok(choco::init());
        } else if cfg!(target_os = "macos") {
            return Ok(brew::init());
        } else if cfg!(target_os = "linux") {
            let release =
                fs::read_to_string("/etc/os-release").map_err(|_| UptError::NotSupportOS)?;
            let id = release
                .lines()
                .find(|l| l.starts_with("ID="))
                .ok_or(UptError::NotSupportOS)?;
            let id = id[3..].trim_matches('"');
            let vendor = match id {
                "arch" | "manjaro" => pacman::init(),
                "centos" | "redhat" | "rhel" => yum::init(),
                "fedora" => dnf::init(),
                "alpine" => apk::init(),
                "debian" | "ubuntu" | "pop-os" | "deepin" | "elementary OS" | "kali"
                | "linuxmint" => apt::init(),
                _ => {
                    return Err(UptError::NotSupportOS);
                }
            };
            return Ok(vendor);
        }
        Err(UptError::NotSupportOS)
    }
    /// Parse command line, figure out the task to perform
    pub fn parse(&self, args: &[String]) -> Result<Task, UptError> {
        self.check_args(args)?;
        if let Some((Some(pkg), yes)) = self.install.parse(args, &self.yes) {
            return Ok(Task::Install { pkg, yes });
        }
        if let Some((Some(pkg), yes)) = self.remove.parse(args, &self.yes) {
            return Ok(Task::Remove { pkg, yes });
        }
        if let Some((Some(pkg), yes)) = self.upgrade.parse(args, &self.yes) {
            return Ok(Task::Upgrade { pkg, yes });
        }
        if let Some((Some(pkg), _)) = self.search.parse(args, &[]) {
            return Ok(Task::Search { pkg });
        }
        if let Some((Some(pkg), _)) = self.show.parse(args, &[]) {
            return Ok(Task::Show { pkg });
        }
        if self.update_index.parse(args, &[]).is_some() {
            return Ok(Task::UpdateIndex);
        }
        if let Some((_, yes)) = self.upgrade_all.parse(args, &self.yes) {
            return Ok(Task::UpgradeAll { yes });
        }
        if self.list_upgradable.parse(args, &[]).is_some() {
            return Ok(Task::ListUpgradable);
        }
        if self.list_installed.parse(args, &[]).is_some() {
            return Ok(Task::ListInstalled);
        }
        Err(UptError::NotRecognize)
    }
    /// Convert the task to command line, which invokes the os's package management tool.
    pub fn eval(&self, task: &Task) -> Result<String, UptError> {
        let cmd = match task {
            Task::Install { pkg, yes } => self.install.to_cmd(pkg, self.yes_str(yes)),
            Task::Remove { pkg, yes } => self.remove.to_cmd(pkg, self.yes_str(yes)),
            Task::Upgrade { pkg, yes } => self.upgrade.to_cmd(pkg, self.yes_str(yes)),
            Task::Search { pkg } => self.search.to_cmd(pkg, ""),
            Task::Show { pkg } => self.show.to_cmd(pkg, ""),
            Task::UpdateIndex => self.update_index.to_cmd("", ""),
            Task::UpgradeAll { yes } => self.upgrade_all.to_cmd("", self.yes_str(yes)),
            Task::ListInstalled => self.list_installed.to_cmd("", ""),
            Task::ListUpgradable => self.list_upgradable.to_cmd("", ""),
        };
        match cmd {
            None => Err(UptError::NotSupportTask),
            Some(cmd) => Ok([self.name.clone(), cmd].join(" ")),
        }
    }
    fn yes_str(&self, yes: &bool) -> &str {
        if !*yes || self.yes.is_empty() {
            return "";
        }
        &self.yes[0]
    }
    /// Dump help message
    pub fn help(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        lines.push(String::new());
        lines.push(String::from("Usage: "));
        let helps = vec![
            (self.install.help(), "Install packages"),
            (self.remove.help(), "Remove packages"),
            (self.upgrade.help(), "Upgrade packages"),
            (self.search.help(), "Search for packages"),
            (self.show.help(), "Show package details"),
            (self.update_index.help(), "Update package indexes"),
            (self.upgrade_all.help(), "Upgrade all packages"),
            (self.list_upgradable.help(), "List all upgradable packages"),
            (self.list_installed.help(), "List all installed packages"),
        ];
        let helps: Vec<(&String, &str)> = helps
            .iter()
            .filter(|(v, _)| v.is_some())
            .map(|(v, d)| (v.as_ref().unwrap(), *d))
            .collect();
        let width = helps.iter().map(|(v, _)| v.len()).max().unwrap() + 6;
        for (cmd, description) in &helps {
            lines.push(format!(
                "  {} {:<width$} {}",
                self.name,
                cmd,
                description,
                width = width
            ));
        }
        if !self.yes.is_empty() {
            lines.push(String::new());
            lines.push(format!("Automatically answer yes: {}", self.yes.join(",")));
            lines.push(String::new());
        }
        lines.join(LINE_ENDING)
    }
    fn check_args(&self, args: &[String]) -> Result<(), UptError> {
        if args.is_empty() {
            return Err(UptError::NotRecognize);
        }
        if args.len() == 1 && args[0].starts_with("--") {
            return Err(UptError::NotRecognize);
        }
        for arg in args {
            if arg == "-" || arg == "--" || arg.starts_with("---") {
                return Err(UptError::BadOption(arg.to_string()));
            }
        }
        Ok(())
    }
}

fn which(name: &str) -> bool {
    Command::new(name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! check_parse {
        ($vendor:expr, [$($arg:expr),*], ($task:tt, $pkg:expr, $yes:expr)) => {
            assert_eq!($vendor.parse(&vec![ $($arg.to_string()),* ]).unwrap(), Task::$task { pkg: $pkg.to_string(),yes: $yes })
        };
        ($vendor:expr, [$($arg:expr),*], ($task:tt, pkg=$pkg:expr)) => {
            assert_eq!($vendor.parse(&vec![ $($arg.to_string()),* ]).unwrap(), Task::$task { pkg: $pkg.to_string() })
        };
        ($vendor:expr, [$($arg:expr),*], ($task:tt, yes=$yes:expr)) => {
            assert_eq!($vendor.parse(&vec![ $($arg.to_string()),* ]).unwrap(), Task::$task { yes: $yes })
        };
        ($vendor:expr, [$($arg:expr),*], $task:tt) => {
            assert_eq!($vendor.parse(&vec![ $($arg.to_string()),* ]).unwrap(), Task::$task)
        };
        ($vendor:expr, [$($arg:expr),*]) => {
            assert!($vendor.parse(&vec![ $($arg.to_string()),* ]).is_err())
        }
    }
    #[test]
    fn test_parse() {
        let upt = upt::init();
        check_parse!(upt, ["install", "vim"], (Install, "vim", false));
        check_parse!(upt, ["install", "-y", "vim"], (Install, "vim", true));
        check_parse!(upt, ["install", "--yes", "vim"], (Install, "vim", true));
        check_parse!(
            upt,
            ["remove", "--yes", "vim", "jq"],
            (Remove, "vim jq", true)
        );
        check_parse!(upt, ["upgrade", "vim"], (Upgrade, "vim", false));
        check_parse!(upt, ["search", "vim"], (Search, pkg = "vim"));
        check_parse!(upt, ["search", "vim", "jq"], (Search, pkg = "vim jq"));
        check_parse!(upt, ["show", "vim"], (Show, pkg = "vim"));
        check_parse!(upt, ["update"], UpdateIndex);
        check_parse!(upt, ["upgrade"], (UpgradeAll, yes = false));
        check_parse!(upt, ["list", "--upgradable"], ListUpgradable);
        check_parse!(upt, ["list", "-i"], ListInstalled);
        check_parse!(upt, ["install"]);
        check_parse!(upt, ["install", "--ye"]);
        check_parse!(upt, ["update", "--yes"]);
        check_parse!(upt, ["list"]);
    }
    macro_rules! check_eval {
        ($vendor:expr, ($task:tt, $pkg:expr, $yes:expr), $cmd:expr) => {
            assert_eq!(
                $vendor
                    .eval(&Task::$task {
                        pkg: $pkg.to_string(),
                        yes: $yes
                    })
                    .unwrap(),
                $cmd.to_string()
            )
        };
        ($vendor:expr, ($task:tt, pkg=$pkg:expr), $cmd:expr) => {
            assert_eq!(
                $vendor
                    .eval(&Task::$task {
                        pkg: $pkg.to_string()
                    })
                    .unwrap(),
                $cmd.to_string()
            )
        };
        ($vendor:expr, ($task:tt, yes=$yes:expr), $cmd:expr) => {
            assert_eq!(
                $vendor.eval(&Task::$task { yes: $yes }).unwrap(),
                $cmd.to_string()
            )
        };
        ($vendor:expr, $task:tt, $cmd:expr) => {
            assert_eq!($vendor.eval(&Task::$task).unwrap(), $cmd.to_string())
        };
        ($vendor:expr) => {
            assert!($vendor.eval(&Task::$task).is_none())
        };
    }
    #[test]
    fn test_eval() {
        let upt = upt::init();
        check_eval!(upt, (Install, "vim", false), "upt install vim");
        check_eval!(upt, (Install, "vim jq", true), "upt install -y vim jq");
        check_eval!(upt, (Remove, "vim jq", false), "upt remove vim jq");
        check_eval!(upt, (Upgrade, "vim", true), "upt upgrade -y vim");
        check_eval!(upt, (Search, pkg = "vim"), "upt search vim");
        check_eval!(upt, (Show, pkg = "vim"), "upt show vim");
        check_eval!(upt, UpdateIndex, "upt update");
        check_eval!(upt, (UpgradeAll, yes = false), "upt upgrade");
        check_eval!(upt, ListInstalled, "upt list -i");
        check_eval!(upt, ListUpgradable, "upt list -u");
    }
}
