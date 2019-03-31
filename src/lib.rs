mod parser;
mod upt;

pub mod error;

use error::UptError;
use parser::Parser;

/// Abstract of a kind of package management tool. e.g. apt, pacman, yum...
pub trait Vender {
    fn name(&self) -> String;
    fn parser_install(&self) -> &Parser;
    fn parser_uninstall(&self) -> &Parser;
    fn parser_upgrade(&self) -> &Parser;
    fn parser_search(&self) -> &Parser;
    fn parser_show(&self) -> &Parser;
    fn parser_update_index(&self) -> &Parser;
    fn parser_upgrade_all(&self) -> &Parser;
    fn parser_list_upgradable(&self) -> &Parser;
    fn parser_list_installed(&self) -> &Parser;
    /// parse command line, figure out the task to perform
    fn parse(&self, args: &[String]) -> Result<Task, UptError> {
        self.check_args(args)?;
        if let Some((Some(pkg), assume_yes)) = self.parser_install().parse(args) {
            return Ok(Task::Install { pkg, assume_yes });
        }
        if let Some((Some(pkg), assume_yes)) = self.parser_uninstall().parse(args) {
            return Ok(Task::Uninstall { pkg, assume_yes });
        }
        if let Some((Some(pkg), assume_yes)) = self.parser_upgrade().parse(args) {
            return Ok(Task::Upgrade { pkg, assume_yes });
        }
        if let Some((Some(pkg), _)) = self.parser_search().parse(args) {
            return Ok(Task::Search { pkg });
        }
        if let Some((Some(pkg), _)) = self.parser_show().parse(args) {
            return Ok(Task::Show { pkg });
        }
        if let Some(_) = self.parser_update_index().parse(args) {
            return Ok(Task::UpdateIndex);
        }
        if let Some(_) = self.parser_upgrade_all().parse(args) {
            return Ok(Task::UpgradeAll);
        }
        if let Some(_) = self.parser_list_upgradable().parse(args) {
            return Ok(Task::ListUpgradable);
        }
        if let Some(_) = self.parser_list_installed().parse(args) {
            return Ok(Task::ListInstalled);
        }
        Err(UptError::NotRecongize)
    }
    /// convert the task to command line, which invokes the os's package management tool.
    fn eval(&self, task: &Task) -> String {
        let cmd = match task {
            Task::Install { pkg, assume_yes } => self
                .parser_install()
                .generate_cmd(&Some(pkg.to_string()), *assume_yes),
            Task::Uninstall { pkg, assume_yes } => self
                .parser_uninstall()
                .generate_cmd(&Some(pkg.to_string()), *assume_yes),
            Task::Upgrade { pkg, assume_yes } => self
                .parser_upgrade()
                .generate_cmd(&Some(pkg.to_string()), *assume_yes),
            Task::Search { pkg } => self
                .parser_search()
                .generate_cmd(&Some(pkg.to_string()), false),
            Task::Show { pkg } => self
                .parser_show()
                .generate_cmd(&Some(pkg.to_string()), false),
            Task::UpdateIndex => self
                .parser_update_index()
                .generate_cmd(&None, false),
            Task::UpgradeAll => self
                .parser_upgrade_all()
                .generate_cmd(&None, false),
            Task::ListInstalled => self
                .parser_list_installed()
                .generate_cmd(&None, false),
            Task::ListUpgradable => self
                .parser_list_upgradable()
                .generate_cmd(&None, false),
        };
        self.name() + " " + &cmd
    }
    fn help(&self, err: &UptError)  {

    }
    fn check_args(&self, args: &[String]) -> Result<(), UptError> {
        if args.len() == 0 {
            return Err(UptError::InvalidArgs);
        }
        for arg in args {
            if arg == "-" || arg == "--" || arg.starts_with("---") {
                return Err(UptError::InvalidArgs);
            }
        }
        Ok(())
    }
}

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

/// lookup vender by name
pub fn lookup_vender(key: &str) -> Result<Box<dyn Vender>, UptError> {
    unimplemented!()
}

/// detect os package management
pub fn detect_os_vender() -> Result<Box<dyn Vender>, UptError> {
    unimplemented!()
}
