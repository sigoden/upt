use crate::error::UptError;
use crate::parser::Parser;
use crate::Task;

pub mod apt;
pub mod pacman;
pub mod upt;

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";
/// Repersent a kind of package management tool. e.g. apt, pacman, yum...
#[derive(Debug)]
pub struct Vendor {
    pub name: String,
    pub install: Parser,
    pub remove: Parser,
    pub upgrade: Parser,
    pub search: Parser,
    pub show: Parser,
    pub update_index: Parser,
    pub upgrade_all: Parser,
    pub list_upgradable: Parser,
    pub list_installed: Parser,
}

/// Abstract of a kind of package management tool. e.g. apt, pacman, yum...
impl Vendor {
    /// Parse command line, figure out the task to perform
    pub fn parse(&self, args: &[String]) -> Result<Task, UptError> {
        self.check_args(args)?;
        if let Some((Some(pkg), assume_yes)) = self.install.parse(args) {
            return Ok(Task::Install { pkg, assume_yes });
        }
        if let Some((Some(pkg), assume_yes)) = self.remove.parse(args) {
            return Ok(Task::Remove { pkg, assume_yes });
        }
        if let Some((Some(pkg), assume_yes)) = self.upgrade.parse(args) {
            return Ok(Task::Upgrade { pkg, assume_yes });
        }
        if let Some((Some(pkg), _)) = self.search.parse(args) {
            return Ok(Task::Search { pkg });
        }
        if let Some((Some(pkg), _)) = self.show.parse(args) {
            return Ok(Task::Show { pkg });
        }
        if self.update_index.parse(args).is_some() {
            return Ok(Task::UpdateIndex);
        }
        if self.upgrade_all.parse(args).is_some() {
            return Ok(Task::UpgradeAll);
        }
        if self.list_upgradable.parse(args).is_some() {
            return Ok(Task::ListUpgradable);
        }
        if self.list_installed.parse(args).is_some() {
            return Ok(Task::ListInstalled);
        }
        Err(UptError::NotRecongize)
    }
    /// Convert the task to command line, which invokes the os's package management tool.
    pub fn eval(&self, task: &Task) -> Result<String, UptError> {
        let cmd = match task {
            Task::Install { pkg, assume_yes } => self
                .install
                .generate_cmd(&Some(pkg.to_string()), *assume_yes),
            Task::Remove { pkg, assume_yes } => self
                .remove
                .generate_cmd(&Some(pkg.to_string()), *assume_yes),
            Task::Upgrade { pkg, assume_yes } => self
                .upgrade
                .generate_cmd(&Some(pkg.to_string()), *assume_yes),
            Task::Search { pkg } => self.search.generate_cmd(&Some(pkg.to_string()), false),
            Task::Show { pkg } => self.show.generate_cmd(&Some(pkg.to_string()), false),
            Task::UpdateIndex => self.update_index.generate_cmd(&None, false),
            Task::UpgradeAll => self.upgrade_all.generate_cmd(&None, false),
            Task::ListInstalled => self.list_installed.generate_cmd(&None, false),
            Task::ListUpgradable => self.list_upgradable.generate_cmd(&None, false),
        };
        if cmd == "" {
            return Err(UptError::NotSupportTask);
        }
        Ok(self.name.clone() + " " + &cmd)
    }
    /// Dump help message
    pub fn help(&self) -> String {
        let mut output = String::new();
        output.push_str(LINE_ENDING);
        output.push_str("Usage:");
        output.push_str(LINE_ENDING);
        let install_help = self.install.generate_help();
        let remove_help = self.remove.generate_help();
        let upgrade_help = self.upgrade.generate_help();
        let search_help = self.search.generate_help();
        let show_help = self.show.generate_help();
        let update_index_help = self.update_index.generate_help();
        let upgrade_all_help = self.upgrade_all.generate_help();
        let list_upgradable_help = self.list_upgradable.generate_help();
        let list_installed_help = self.list_installed.generate_help();
        let widths = vec![
            install_help.len(),
            remove_help.len(),
            upgrade_help.len(),
            search_help.len(),
            show_help.len(),
            update_index_help.len(),
            upgrade_all_help.len(),
            list_upgradable_help.len(),
            list_installed_help.len(),
        ];
        let max_width = widths.iter().max().unwrap() + 6;
        let head = "  ".to_string() + &self.name + " ";
        if !install_help.is_empty() {
            output.push_str(&format!(
                "{}{:<width$} Install packages",
                head,
                install_help,
                width = max_width
            ));
            output.push_str(LINE_ENDING);
        }
        if !remove_help.is_empty() {
            output.push_str(&format!(
                "{}{:<width$} Remove packages",
                head,
                remove_help,
                width = max_width
            ));
            output.push_str(LINE_ENDING);
        }
        if !upgrade_help.is_empty() {
            output.push_str(&format!(
                "{}{:<width$} Upgrade packages",
                head,
                upgrade_help,
                width = max_width
            ));
            output.push_str(LINE_ENDING);
        }
        if !search_help.is_empty() {
            output.push_str(&format!(
                "{}{:<width$} Search for package",
                head,
                search_help,
                width = max_width
            ));
            output.push_str(LINE_ENDING);
        }
        if !show_help.is_empty() {
            output.push_str(&format!(
                "{}{:<width$} Show package details",
                head,
                show_help,
                width = max_width
            ));
            output.push_str(LINE_ENDING);
        }
        if !update_index_help.is_empty() {
            output.push_str(&format!(
                "{}{:<width$} Update indexes of packages",
                head,
                update_index_help,
                width = max_width
            ));
            output.push_str(LINE_ENDING);
        }
        if !upgrade_all_help.is_empty() {
            output.push_str(&format!(
                "{}{:<width$} Upgrade all packages",
                head,
                upgrade_all_help,
                width = max_width
            ));
            output.push_str(LINE_ENDING);
        }
        if !list_upgradable_help.is_empty() {
            output.push_str(&format!(
                "{}{:<width$} List all upgradable packages",
                head,
                list_upgradable_help,
                width = max_width
            ));
            output.push_str(LINE_ENDING);
        }
        if !list_installed_help.is_empty() {
            output.push_str(&format!(
                "{}{:<width$} List all installed packages",
                head,
                list_installed_help,
                width = max_width
            ));
            output.push_str(LINE_ENDING);
        }
        output
    }
    fn check_args(&self, args: &[String]) -> Result<(), UptError> {
        if args.is_empty() {
            return Err(UptError::NoSubcommand);
        }
        for arg in args {
            if arg == "-" || arg == "--" || arg.starts_with("---") {
                return Err(UptError::BadOption(arg.to_string()));
            }
        }
        Ok(())
    }
    /// Lookup vender by name
    pub fn lookup(name: &str) -> Result<Vendor, UptError> {
        match name {
            "upt" => return Ok(upt::init()),
            "apt" => return Ok(apt::init()),
            "pacman" => return Ok(pacman::init()),
            _ => {}
        }
        Err(UptError::NoVendor(name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! check_parse {
        ($parser:expr, [$($arg:expr),*], ($task:tt, $pkg:expr, $assume_yes:expr)) => {
            assert_eq!($parser.parse(&vec![ $($arg.to_string()),* ]).unwrap(), Task::$task { pkg: $pkg.to_string(),assume_yes: $assume_yes })
        };
        ($parser:expr, [$($arg:expr),*], ($task:tt, pkg=$pkg:expr)) => {
            assert_eq!($parser.parse(&vec![ $($arg.to_string()),* ]).unwrap(), Task::$task { pkg: $pkg.to_string() })
        };
        ($parser:expr, [$($arg:expr),*], ($task:tt, assume_yes=$assume_yes:expr)) => {
            assert_eq!($parser.parse(&vec![ $($arg.to_string()),* ]).unwrap(), Task::$task { assume_yes: $assume_yes })
        };
        ($parser:expr, [$($arg:expr),*], $task:tt) => {
            assert_eq!($parser.parse(&vec![ $($arg.to_string()),* ]).unwrap(), Task::$task)
        };
        ($parser:expr, [$($arg:expr),*]) => {
            assert!($parser.parse(&vec![ $($arg.to_string()),* ]).is_err())
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
        check_parse!(upt, ["upgrade"], UpgradeAll);
        check_parse!(upt, ["list", "--upgradable"], ListUpgradable);
        check_parse!(upt, ["list", "-i"], ListInstalled);
        check_parse!(upt, ["install"]);
        check_parse!(upt, ["install", "--ye"]);
        check_parse!(upt, ["update", "--yes"]);
        check_parse!(upt, ["list"]);
    }
    macro_rules! check_eval {
        ($parser:expr, ($task:tt, $pkg:expr, $assume_yes:expr), $cmd:expr) => {
            assert_eq!(
                $parser
                    .eval(&Task::$task {
                        pkg: $pkg.to_string(),
                        assume_yes: $assume_yes
                    })
                    .unwrap(),
                $cmd.to_string()
            )
        };
        ($parser:expr, ($task:tt, pkg=$pkg:expr), $cmd:expr) => {
            assert_eq!(
                $parser
                    .eval(&Task::$task {
                        pkg: $pkg.to_string()
                    })
                    .unwrap(),
                $cmd.to_string()
            )
        };
        ($parser:expr, ($task:tt, assume_yes=$assume_yes:expr), $cmd:expr) => {
            assert_eq!(
                $parser
                    .eval(&Task::$task {
                        assume_yes: $assume_yes
                    })
                    .unwrap(),
                $cmd.to_string()
            )
        };
        ($parser:expr, $task:tt, $cmd:expr) => {
            assert_eq!($parser.eval(&Task::$task).unwrap(), $cmd.to_string())
        };
        ($parser:expr) => {
            assert!($parser.eval(&Task::$task).is_err())
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
        check_eval!(upt, UpgradeAll, "upt upgrade");
        check_eval!(upt, ListInstalled, "upt list -i");
        check_eval!(upt, ListUpgradable, "upt list -u");
    }
}
