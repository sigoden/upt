use crate::error::UptError;
use crate::parser::Parser;
use crate::Task;

pub mod apt;
/// Repersent a kind of package management tool. e.g. apt, pacman, yum...
#[derive(Debug)]
pub struct Vendor {
    pub name: String,
    pub install: Parser,
    pub uninstall: Parser,
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
        if let Some((Some(pkg), assume_yes)) = self.uninstall.parse(args) {
            return Ok(Task::Uninstall { pkg, assume_yes });
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
        if let Some(_) = self.update_index.parse(args) {
            return Ok(Task::UpdateIndex);
        }
        if let Some(_) = self.upgrade_all.parse(args) {
            return Ok(Task::UpgradeAll);
        }
        if let Some(_) = self.list_upgradable.parse(args) {
            return Ok(Task::ListUpgradable);
        }
        if let Some(_) = self.list_installed.parse(args) {
            return Ok(Task::ListInstalled);
        }
        Err(UptError::NotRecongize)
    }
    /// Convert the task to command line, which invokes the os's package management tool.
    pub fn eval(&self, task: &Task) -> String {
        let cmd = match task {
            Task::Install { pkg, assume_yes } => self
                .install
                .generate_cmd(&Some(pkg.to_string()), *assume_yes),
            Task::Uninstall { pkg, assume_yes } => self
                .uninstall
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
        self.name.clone() + " " + &cmd
    }
    /// Dump help message
    pub fn help(&self) -> String {
        let mut output = String::new();
        let install_help = self.install.generate_help();
        let uninstall_help = self.uninstall.generate_help();
        let upgrade_help = self.upgrade.generate_help();
        let search_help = self.search.generate_help();
        let show_help = self.show.generate_help();
        let update_index_help = self.update_index.generate_help();
        let upgrade_all_help = self.upgrade_all.generate_help();
        let list_upgradable_help = self.list_upgradable.generate_help();
        let list_installed_help = self.list_installed.generate_help();
        let widths = vec![
            install_help.len(),
            uninstall_help.len(),
            upgrade_help.len(),
            search_help.len(),
            show_help.len(),
            update_index_help.len(),
            upgrade_all_help.len(),
            list_upgradable_help.len(),
            list_installed_help.len(),
        ];
        let max_width = widths.iter().max().unwrap();
        if install_help.len() > 0 {
            output.push_str(&format!("{:<width$} Install packages", install_help, width=max_width));
        }
        if uninstall_help.len() > 0 {
            output.push_str(&format!("{:<width$} Uninstall packages", uninstall_help, width=max_width));
        }
        if upgrade_help.len() > 0 {
            output.push_str(&format!("{:<width$} Upgrade packages", upgrade_help, width=max_width));
        }
        if search_help.len() > 0 {
            output.push_str(&format!("{:<width$} Search packages", search_help, width=max_width));
        }
        if show_help.len() > 0 {
            output.push_str(&format!("{:<width$} Show packages", show_help, width=max_width));
        }
        if update_index_help.len() > 0 {
            output.push_str(&format!("{:<width$} Update indexes of packages ", update_index_help, width=max_width));
        }
        if upgrade_all_help.len() > 0 {
            output.push_str(&format!("{:<width$} upgrade all outdated packages", upgrade_all_help, width=max_width));
        }
        if list_upgradable_help.len() > 0 {
            output.push_str(&format!("{:<width$} list all upgradable packages", list_upgradable_help, width=max_width));
        }
        if list_installed_help.len() > 0 {
            output.push_str(&format!("{:<width$} list all installed packages", list_installed_help, width=max_width));
        }
        output
    }
    fn check_args(&self, args: &[String]) -> Result<(), UptError> {
        if args.len() == 0 {
            return Err(UptError::NoSubcommand);
        }
        for arg in args {
            if arg == "-" || arg == "--" || arg.starts_with("---") {
                return Err(UptError::BadOption(arg.to_string()));
            }
        }
        Ok(())
    }
}
