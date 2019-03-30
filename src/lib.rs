use std::str::FromStr;
#[derive(Debug, PartialEq)]
pub enum UptError {
    NotFoundVender,
    Parse(String),
}

// mod upt;

/// Abstract of a kind of package management tool. e.g. apt, pacman, yum...
pub trait Vender {
    /// parse command line, figure out the task to perform
    fn parse(&self, args: &[String]) -> Result<Task, UptError>;
    /// convert the task to command line, which invokes the os's package management tool.
    fn eval(&self, task: Task) -> Vec<String>;
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

/// Command line parser
#[derive(Debug, PartialEq)]
pub struct Parser<'a> {
    command: &'a str,
    assume_yes: Vec<&'a str>,
    have_operands: bool,
    required_options: Vec<Vec<&'a str>>,
}

impl<'a> FromStr for Parser<'a> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let words: Vec<&'a str> = s.split(' ').collect();
        let (command, remind) = words.split_first().expect("no command");
        let mut have_operands = false;
        let mut required_options: Vec<Vec<&str>> = vec![];
        let mut assume_yes: Vec<&str> = Vec::new();
        fn split(v: &str) -> Vec<&str> {
            v.split('|').map(|x| x.trim_start_matches('-')).collect::<Vec<&str>>()
        }
        for elem in remind {
            if elem.starts_with("-") {
                if elem.ends_with("@assume_yes") {
                    assume_yes = split(elem);
                } else {
                    required_options.push(split(elem));
                }
            } else if *elem == "$" {
                have_operands = true;
            }
        }
        // unimplemented!()
        Ok(Parser {
            command,
            assume_yes,
            have_operands,
            required_options,
        })
    }
}


impl<'a> Parser<'a> {
    pub fn parse<F>(&self, args: &[String], create_task: F) -> Option<Result<Task, UptError>>
        where F: FnOnce(Option<String>, bool) -> Option<Result<Task, UptError>> {
        let (command, options, pkg) = Self::grouping_args(args);
        if command != self.command {
            return None;
        }
        if !self.satisfy_required_options(&options) {
            return None;
        }
        let assume_yes = if self.assume_yes.len() > 0 {
            options.iter().find(|v| self.assume_yes.iter().find(|x| x == v).is_some()).is_some()
        } else {
            false
        };
        create_task(pkg, assume_yes)
    }
    fn grouping_args(args: &[String]) -> (&str, Vec<&str>, Option<String>) {
        let mut command = "";
        let mut options: Vec<&str> = vec![];
        let mut operands: Vec<&str> = vec![];
        let mut visit_command = false;
        for arg in args.into_iter() {
            if arg.starts_with("--") {
                options.push(&arg[2..]);
            } else if arg.starts_with("-") {
                for i in 1..arg.len() {
                    options.push(&arg[i..i + 1]);
                }
            } else {
                if visit_command {
                    command = arg;
                } else {
                    operands.push(arg);
                }
            }
            visit_command = true;
        }
        let pkg = if operands.len() > 0 {
            Some(operands.join(" "))
        } else {
            None
        };
        (command, options, pkg)
    }
    fn satisfy_required_options(&self, options: &Vec<&str>) -> bool {
        let mut coll: Vec<(usize, &&str)> = options.iter().enumerate().collect();
        for ro in self.required_options.iter() {
            match coll
                .iter()
                .find(|(_, v)| ro.iter().find(|x| x == v).is_some())
            {
                None => return false,
                Some((i, _)) => coll.remove(*i),
            };
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use std::str::FromStr;

    macro_rules! check_parser_from_str {
        ($input:expr, { $command:expr, [$($ay:expr),*], [$($ro:expr),*], $op:expr }) => {
            let parser = Parser::from_str($input).unwrap();
            let expect_parser = Parser {
                command: $command,
                assume_yes: vec![ $($ay),*],
                have_operands: $op,
                required_options: vec![ $($ro),*],
            }
            assert_eq!(parser, expect_parser);
        }
    }
    #[test]
    fn test_parser_from_str() {
        check_parser_from_str!(
            "install @assume_yes(y|yes) $",
            { "install", ["y", "yes"], [], true }
        );
        // check_parser_from_str!(
        //     "list --installed|-i --yes|-y@assume_yes $",
        //     { "install", ["y", "yes"], [], true }
        // );
    }
}
