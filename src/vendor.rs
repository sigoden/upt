use crate::error::UptError;
use crate::subcommand::SubCommand;
use crate::task::Task;

os_tools!(
  "windows" => "scoop", "choco", "winget";
  "macos" => "brew", "port";
  // apt
  "ubuntu" => "apt";
  "debian" => "apt";
  "linuxmint" => "apt";
  "pop" => "apt";
  "deepin" => "apt";
  "elementray" => "apt";
  "kali" => "apt";
  "raspbian" => "apt";
  "aosc" => "apt";
  "zorin" => "apt";
  "antix" => "apt";
  "devuan" => "apt";
  "bodhi" => "apt";
  "lxle" => "apt";
  // dnf
  "fedora" => "dnf", "yum";
  "redhat" => "dnf", "yum";
  "rhel" => "dnf", "yum";
  "amzn" => "dnf", "yum";
  "ol" => "dnf", "yum";
  "almalinux" => "dnf", "yum";
  "rocky" => "dnf", "yum";
  "oubes" => "dnf", "yum";
  "centos" => "dnf", "yum";
  "qubes" => "dnf", "yum";
  "eurolinux" => "dnf", "yum";
  // pacman
  "arch" => "pacman";
  "manjaro" => "pacman";
  "endeavouros" => "pacman";
  "arcolinux" => "pacman";
  "garuda" => "pacman";
  "antergos" => "pacman";
  "kaos" => "pacman";
  // apk
  "alpine" => "apk";
  "postmarket" => "apk";
  // zypper
  "opensuse" => "zypper";
  "opensuse-leap" => "zypper";
  "opensuse-tumbleweed" => "zypper";
  // nix
  "nixos" => "nix-env";
  // emerge
  "gentoo" => "emerge";
  "funtoo" => "emerge";
  // xps
  "void" => "xbps";
  // urpm
  "mageia" => "urpm";
  // slackpkg
  "slackware" => "slackpkg";
  // eopkg
  "solus" => "eopkg";
  // opkg
  "openwrt" => "opkg";
  // cards
  "nutyx" => "cards";
  // pkg
  "freebsd" => "pkg";
  "ghostbsd" => "pkg";
  // pkg(2)
  "andorid" => "pkg(2)"
);

vendors![
    {
        name: "apk",
        confirm: "",
        install: "apk add $",
        remove: "apk del $",
        upgrade: "apk upgrade $",
        search: "apk search $",
        info: "apk show $",
        update_index: "apk update",
        upgrade_all: "apk upgrade",
        list_installed: "apk list -I/--installed",
    },
    {
        name: "apt",
        confirm: "-y/--yes",
        install: "apt install $",
        remove: "apt remove $",
        upgrade: "apt install --only-upgrade $",
        search: "apt search $",
        info: "apt show $",
        update_index: "apt update",
        upgrade_all: "apt upgrade",
        list_installed: "apt list -i/--installed",
    },
    {
        name: "brew",
        confirm: "",
        install: "brew install $",
        remove: "brew uninstall $",
        upgrade: "brew upgrade $",
        search: "brew search $",
        info: "brew info $",
        update_index: "brew update",
        upgrade_all: "brew upgrade",
        list_installed: "brew list",
    },
    {
        name: "cards",
        confirm: "",
        install: "cards install $",
        remove: "cards remove $",
        upgrade: "cards install -u/--upgrade $",
        search: "cards search $",
        info: "cards info $",
        update_index: "cards sync",
        upgrade_all: "cards upgrade",
        list_installed: "cards list",
    },
    {
        name: "choco",
        confirm: "-y",
        install: "choco install $",
        remove: "choco uninstall $",
        upgrade: "choco upgrade $",
        search: "choco search $",
        info: "choco info $",
        update_index: "choco upgrade all --noop",
        upgrade_all: "choco upgrade all",
        list_installed: "choco list -l/--local-only",
    },
    {
        name: "dnf",
        confirm: "-y/--assumeyes",
        install: "dnf install $",
        remove: "dnf remove $",
        upgrade: "dnf upgrade $",
        search: "dnf search $",
        info: "dnf info $",
        update_index: "dnf check-update",
        upgrade_all: "dnf update",
        list_installed: "dnf list --installed",
    },
    {
        name: "emerge",
        confirm: "",
        install: "emerge $",
        remove: "emerge --deselect $",
        upgrade: "emerge --update $",
        search: "emerge --search $",
        info: "emerge --info $",
        update_index: "emerge --sync",
        upgrade_all: "emerge -vuDN @world",
        list_installed: "qlist -Iv",
    },
    {
        name: "eopkg",
        confirm: "-y/--yes-all",
        install: "eopkg install $",
        remove: "eopkg remove $",
        upgrade: "eopkg upgrade $",
        search: "eopkg search $",
        info: "eopkg info $",
        update_index: "eopkg update-repo",
        upgrade_all: "eopkg upgrade",
        list_installed: "eopkg list-installed",
    },
    {
        name: "flatpak",
        confirm: " -y/--assumeyes",
        install: "flatpak install $",
        remove: "flatpak uninstall $",
        upgrade: "flatpak update $",
        search: "flatpak search $",
        info: "flatpak info $",
        update_index: "",
        upgrade_all: "flatpak update",
        list_installed: "flatpak list",
    },
    {
        name: "guix",
        confirm: "",
        install: "guix install $",
        remove: "guix remove $",
        upgrade: "guix upgrade $",
        search: "guix search $",
        info: "guix show $",
        update_index: "guix refresh",
        upgrade_all: "guix upgrade",
        list_installed: "guix package -I/--list-installed",
    },
    {
        name: "nix-env",
        confirm: "",
        install: "nix-env -i/--install $",
        remove: "pacman -e/--uninstall $",
        upgrade: "nix-env -u/--upgrade $",
        search: "nix-env -qaP $",
        info: "nix-env -qa --description $",
        update_index: "nix-channel --update",
        upgrade_all: "nix-env -u/--upgrade",
        list_installed: "nix-env -q/--query --installed",
    },
    {
        name: "opkg",
        confirm: "",
        install: "opkg install $",
        remove: "opkg remove $",
        upgrade: "opkg upgrade $",
        search: "opkg find $",
        info: "opkg info $",
        update_index: "opkg update",
        upgrade_all: "opkg upgrade",
        list_installed: "opkg list-installed",
    },
    {
        name: "pacman",
        confirm: "--noconfirm",
        install: "pacman -S $",
        remove: "pacman -R -s $",
        upgrade: "pacman -S $",
        search: "pacman -S -s $",
        info: "pacman -S -i $",
        update_index: "pacman -S -y -y",
        upgrade_all: "pacman -S -y -u",
        list_installed: "pacman -Q -e",
    },
    {
        name: "pkg",
        confirm: "-y/--yes",
        install: "pkg install $",
        remove: "pkg remove $",
        upgrade: "pkg install $",
        search: "pkg search $",
        info: "pkg info $",
        update_index: "pkg update",
        upgrade_all: "pkg upgrade",
        list_installed: "pkg info -a/--all",
    },
    {
        name: "pkg(2)",
        confirm: "-y/--yes",
        install: "pkg install $",
        remove: "pkg uninstall $",
        upgrade: "pkg install $",
        search: "pkg search $",
        info: "pkg show $",
        update_index: "pkg update",
        upgrade_all: "pkg upgrade",
        list_installed: "pkg list-installed",
    },
    {
        name: "scoop",
        confirm: "",
        install: "scoop install $",
        remove: "scoop uninstall $",
        upgrade: "scoop update $",
        search: "scoop search $",
        info: "scoop info $",
        update_index: "scoop update",
        upgrade_all: "scoop update *",
        list_installed: "scoop list",
    },
    {
        name: "slackpkg",
        confirm: "",
        install: "slackpkg install $",
        remove: "slackpkg remove $",
        upgrade: "slackpkg upgrade $",
        search: "slackpkg search $",
        info: "slackpkg info $",
        update_index: "slackpkg update",
        upgrade_all: "slackpkg upgrade-all",
        list_installed: "ls -1 /var/log/packages",
    },
    {
        name: "snap",
        confirm: "",
        install: "snap install --classic $",
        remove: "snap remove $",
        upgrade: "snap refresh $",
        search: "snap find $",
        info: "snap info $",
        update_index: "",
        upgrade_all: "snap refresh",
        list_installed: "snap list",
    },
    {
        name: "upt",
        confirm: "-y/--yes",
        install: "upt install $",
        remove: "upt remove $",
        upgrade: "upt upgrade $",
        search: "upt search $",
        info: "upt info $",
        update_index: "upt update",
        upgrade_all: "upt upgrade",
        list_installed: "upt list",
    },
    {
        name: "urpm",
        confirm: "",
        install: "urpmi $",
        remove: "urpme $",
        upgrade: "urpmi $",
        search: "urpmq -y/--fuzzy $",
        info: "urpmq -i $",
        update_index: "urpmi.update -a",
        upgrade_all: "urpmi --auto-update",
        list_installed: "rpm -q/--query --all",
    },
    {
        name: "winget",
        confirm: "",
        install: "winget install $",
        remove: "winget uninstall $",
        upgrade: "winget upgrade $",
        search: "winget search $",
        info: "winget show $",
        update_index: "",
        upgrade_all: "winget upgrade --all",
        list_installed: "winget list",
    },
    {
        name: "xbps",
        confirm: "-y/--yes",
        install: "xbps-install $",
        remove: "xbps-remove $",
        upgrade: "xbps-install -u/--update $",
        search: "xbps-query -Rs $",
        info: "xbps-query -RS $",
        update_index: "xbps-install -S/--sync",
        upgrade_all: "xbps-install -u/--update",
        list_installed: "xbps-query -l/--list-pkgs",
    },
    {
        name: "yum",
        confirm: "-y/--assumeyes",
        install: "yum install $",
        remove: "yum remove $",
        upgrade: "yum update $",
        search: "yum search $",
        info: "yum info $",
        update_index: "yum check-update",
        upgrade_all: "yum update",
        list_installed: "yum list --installed",
    },
    {
        name: "zypper",
        confirm: "-y/--no-confirm",
        install: "zypper install $",
        remove: "zypper remove $",
        upgrade: "zypper update $",
        search: "zypper search $",
        info: "zypper info $",
        update_index: "zypper refresh",
        upgrade_all: "zypper update",
        list_installed: "zypper search -i/--installed-only",
    },
];

/// Represent a kind of package management tool. e.g. apt, pacman, yum...
#[derive(Debug, Clone, PartialEq)]
pub struct Vendor {
    pub(crate) name: String,
    pub(crate) confirm: String,
    pub(crate) install: SubCommand,
    pub(crate) remove: SubCommand,
    pub(crate) upgrade: SubCommand,
    pub(crate) search: SubCommand,
    pub(crate) info: SubCommand,
    pub(crate) update_index: SubCommand,
    pub(crate) upgrade_all: SubCommand,
    pub(crate) list_installed: SubCommand,
}

impl Vendor {
    /// Parse command line, figure out the task to perform
    pub fn parse(&self, args: &[String]) -> Result<Task, UptError> {
        if self.is_help(args) {
            return Err(UptError::DisplyHelp(self.help()));
        }
        if let Some((Some(pkg), yes)) = self.install.parse(args, &self.confirm) {
            return Ok(Task::Install { pkg, confirm: yes });
        }
        if let Some((Some(pkg), yes)) = self.remove.parse(args, &self.confirm) {
            return Ok(Task::Remove { pkg, confirm: yes });
        }
        if let Some((Some(pkg), yes)) = self.upgrade.parse(args, &self.confirm) {
            return Ok(Task::Upgrade { pkg, confirm: yes });
        }
        if let Some((Some(pkg), _)) = self.search.parse(args, "") {
            return Ok(Task::Search { pkg });
        }
        if let Some((Some(pkg), _)) = self.info.parse(args, "") {
            return Ok(Task::Info { pkg });
        }
        if self.update_index.parse(args, "").is_some() {
            return Ok(Task::UpdateIndex);
        }
        if let Some((_, yes)) = self.upgrade_all.parse(args, &self.confirm) {
            return Ok(Task::UpgradeAll { confirm: yes });
        }
        if self.list_installed.parse(args, "").is_some() {
            return Ok(Task::ListInstalled);
        }
        Err(UptError::InvalidArgs(self.help()))
    }

    /// Convert the task to command line, which invokes the os's package management tool.
    pub fn eval(&self, task: &Task) -> Result<String, UptError> {
        let cmd = match task {
            Task::Install { pkg, confirm: yes } => self.install.to_cmd(pkg, self.yes_str(yes)),
            Task::Remove { pkg, confirm: yes } => self.remove.to_cmd(pkg, self.yes_str(yes)),
            Task::Upgrade { pkg, confirm: yes } => self.upgrade.to_cmd(pkg, self.yes_str(yes)),
            Task::Search { pkg } => self.search.to_cmd(pkg, ""),
            Task::Info { pkg } => self.info.to_cmd(pkg, ""),
            Task::UpdateIndex => self.update_index.to_cmd("", ""),
            Task::UpgradeAll { confirm: yes } => self.upgrade_all.to_cmd("", self.yes_str(yes)),
            Task::ListInstalled => self.list_installed.to_cmd("", ""),
        };
        cmd.ok_or(UptError::NotSupportTask)
    }

    fn yes_str(&self, yes: &bool) -> &str {
        if !*yes || self.confirm.is_empty() {
            return "";
        }
        match self.confirm.split_once('/') {
            Some((v, _)) => v,
            None => "",
        }
    }

    fn is_help(&self, args: &[String]) -> bool {
        args.len() < 2
            || args
                .iter()
                .skip(1)
                .any(|arg| ["-h", "--help"].iter().any(|option| option == arg))
    }

    /// Dump help message
    fn help(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        lines.push(String::from("Usage: "));
        let helps = vec![
            (self.install.help(), "Install packages"),
            (self.remove.help(), "Remove packages"),
            (self.upgrade.help(), "Upgrade packages"),
            (self.search.help(), "Search for packages"),
            (self.info.help(), "Show package details"),
            (self.update_index.help(), "Update package indexes"),
            (self.upgrade_all.help(), "Upgrade all packages"),
            (self.list_installed.help(), "List all installed packages"),
        ];
        let helps: Vec<(&String, &str)> = helps
            .iter()
            .filter(|(v, _)| v.is_some())
            .map(|(v, d)| (v.as_ref().unwrap(), *d))
            .collect();
        let width = helps.iter().map(|(v, _)| v.len()).max().unwrap() + 6;
        for (cmd, description) in &helps {
            lines.push(format!("  {:<width$} {}", cmd, description, width = width));
        }
        if !self.confirm.is_empty() {
            lines.push(String::new());
            lines.push(format!(
                "Automatically confirm the action with: {}",
                self.confirm
            ));
        }
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! check_parse {
        ($vendor:expr, [$($arg:expr),*], ($task:tt, $pkg:expr, $confirm:expr)) => {
            assert_eq!($vendor.parse(&vec![ $($arg.to_string()),* ]).unwrap(), Task::$task { pkg: $pkg.to_string(), confirm: $confirm })
        };
        ($vendor:expr, [$($arg:expr),*], ($task:tt, pkg=$pkg:expr)) => {
            assert_eq!($vendor.parse(&vec![ $($arg.to_string()),* ]).unwrap(), Task::$task { pkg: $pkg.to_string() })
        };
        ($vendor:expr, [$($arg:expr),*], ($task:tt, confirm=$confirm:expr)) => {
            assert_eq!($vendor.parse(&vec![ $($arg.to_string()),* ]).unwrap(), Task::$task { confirm: $confirm })
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
        let upt = init("upt").unwrap();
        check_parse!(upt, ["upt", "install", "vim"], (Install, "vim", false));
        check_parse!(upt, ["upt", "install", "-y", "vim"], (Install, "vim", true));
        check_parse!(
            upt,
            ["upt", "install", "--yes", "vim"],
            (Install, "vim", true)
        );
        check_parse!(
            upt,
            ["upt", "remove", "--yes", "vim", "jq"],
            (Remove, "vim jq", true)
        );
        check_parse!(upt, ["upt", "upgrade", "vim"], (Upgrade, "vim", false));
        check_parse!(upt, ["upt", "search", "vim"], (Search, pkg = "vim"));
        check_parse!(
            upt,
            ["upt", "search", "vim", "jq"],
            (Search, pkg = "vim jq")
        );
        check_parse!(upt, ["upt", "info", "vim"], (Info, pkg = "vim"));
        check_parse!(upt, ["upt", "update"], UpdateIndex);
        check_parse!(upt, ["upt", "upgrade"], (UpgradeAll, confirm = false));
        check_parse!(upt, ["upt", "list"], ListInstalled);
        check_parse!(upt, ["upt", "install"]);
        check_parse!(upt, ["upt", "install", "--ye"]);
        check_parse!(upt, ["upt", "update", "--yes"]);
    }

    macro_rules! check_eval {
        ($vendor:expr, ($task:tt, $pkg:expr, $confirm:expr), $cmd:expr) => {
            assert_eq!(
                $vendor
                    .eval(&Task::$task {
                        pkg: $pkg.to_string(),
                        confirm: $confirm
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
        ($vendor:expr, ($task:tt, confirm=$confirm:expr), $cmd:expr) => {
            assert_eq!(
                $vendor.eval(&Task::$task { confirm: $confirm }).unwrap(),
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
        let upt = init("upt").unwrap();
        check_eval!(upt, (Install, "vim", false), "upt install vim");
        check_eval!(upt, (Install, "vim jq", true), "upt install -y vim jq");
        check_eval!(upt, (Remove, "vim jq", false), "upt remove vim jq");
        check_eval!(upt, (Upgrade, "vim", true), "upt upgrade -y vim");
        check_eval!(upt, (Search, pkg = "vim"), "upt search vim");
        check_eval!(upt, (Info, pkg = "vim"), "upt info vim");
        check_eval!(upt, UpdateIndex, "upt update");
        check_eval!(upt, (UpgradeAll, confirm = false), "upt upgrade");
        check_eval!(upt, ListInstalled, "upt list");
    }

    #[test]
    fn test_vendors() {
        for tool in support_tools() {
            init(tool).unwrap();
        }
    }
}
