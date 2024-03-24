use crate::UptError;

use std::cell::Cell;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct Action {
    cmd: String,
    subcmd: Vec<String>,
    options: Vec<Vec<String>>,
    args: Vec<String>,
    has_pkg: bool,
}

impl FromStr for Action {
    type Err = UptError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Default::default());
        }
        let words: Vec<&str> = s.split(' ').collect();
        let mut has_pkg = false;
        let mut options: Vec<Vec<String>> = vec![];
        let mut args = vec![];
        if words.len() < 2 {
            return Err(UptError::InvalidAction(s.to_string()));
        }
        let (cmd, subcmd, reminder) = if words[1].starts_with('-') || words[1] == "$" {
            (words[0].to_string(), vec![], &words[1..])
        } else {
            (words[0].to_string(), split(words[1]), &words[2..])
        };
        for elem in reminder {
            if elem == &"$" {
                has_pkg = true;
                continue;
            }
            if elem.starts_with('-') {
                options.push(split(elem));
            } else {
                args.push(elem.to_string());
            }
        }
        Ok(Action {
            cmd,
            subcmd,
            options,
            args,
            has_pkg,
        })
    }
}

impl Action {
    pub fn parse(&self, args: &[String], confirm: &str) -> Option<(Option<String>, bool)> {
        if self.invalid() {
            return None;
        }
        let (options, pkg) = self.parse_args(args)?;
        if (pkg.is_none() && self.has_pkg) || (pkg.is_some() && !self.has_pkg) {
            return None;
        }
        let no_confirm_options: Vec<String> = options
            .iter()
            .filter(|v| !confirm.split('/').any(|y| &y == v))
            .cloned()
            .collect();

        let confirm = no_confirm_options.len() != options.len();
        if !self.satisfy_options(&no_confirm_options) {
            return None;
        }
        Some((pkg, confirm))
    }

    pub fn to_cmd(&self, pkg: &str, confirm: &str) -> Option<String> {
        if self.invalid() {
            if pkg.is_empty() {
                return Some("".to_string());
            } else {
                return None;
            }
        }
        let mut segs: Vec<&str> = vec![&self.cmd];
        if let Some(action) = &self.subcmd.first() {
            segs.push(action);
        }
        for item in &self.options {
            segs.push(&item[0]);
        }
        if !self.args.is_empty() {
            segs.extend(self.args.iter().map(|v| v.as_str()));
        }
        if !pkg.is_empty() {
            segs.push(pkg);
        }
        if !confirm.is_empty() {
            segs.push(confirm);
        }
        Some(segs.join(" "))
    }

    pub fn help(&self) -> Option<String> {
        if self.invalid() {
            return None;
        }
        let mut segs: Vec<String> = vec![self.cmd.clone()];

        if !self.subcmd.is_empty() {
            segs.push(join(&self.subcmd));
        }

        for item in &self.options {
            if item.len() > 1 {
                segs.push(join(item));
            } else {
                segs.push(item[0].clone());
            }
        }
        if self.has_pkg {
            segs.push(String::from("<pkg>"));
        }
        Some(segs.join(" "))
    }

    fn invalid(&self) -> bool {
        self.cmd.is_empty()
    }

    fn parse_args(&self, args: &[String]) -> Option<(Vec<String>, Option<String>)> {
        if args.len() < 2 {
            return None;
        }
        if self.cmd != args[0] {
            return None;
        }
        let reminder = if self.subcmd.is_empty() {
            &args[1..]
        } else {
            if !self.subcmd.contains(&args[1]) {
                return None;
            }
            &args[2..]
        };
        let mut options: Vec<String> = vec![];
        let mut operands: Vec<String> = vec![];
        for arg in reminder.iter() {
            if arg.starts_with("--") {
                options.push(arg.to_string());
            } else if arg.starts_with('-') {
                // split combined short options, -Syy => ["-S", "-y", "-y"]
                for i in 1..arg.len() {
                    let single_arg = "-".to_string() + &arg[i..=i];
                    options.push(single_arg);
                }
            } else {
                operands.push(arg.to_string());
            }
        }
        let pkg = if operands.is_empty() {
            None
        } else {
            Some(operands.join(" "))
        };
        Some((options, pkg))
    }

    fn satisfy_options(&self, options: &[String]) -> bool {
        let marks: Vec<Cell<bool>> = self.options.iter().map(|_| Cell::new(false)).collect();
        if options.len() != self.options.len() {
            return false;
        }
        options.iter().all(|v| {
            self.options.iter().enumerate().any(|(i, y)| {
                let mark = marks.get(i).unwrap();
                if !mark.get() && y.iter().any(|z| z == v) {
                    mark.set(true);
                    return true;
                }
                false
            })
        })
    }
}

/// used in vendor!
pub(crate) fn must_from_str(s: &str, name: &str, field: &str) -> Action {
    match Action::from_str(s) {
        Ok(p) => p,
        Err(_) => panic!("Failed to parse {}.{} from '{}' ", name, field, s),
    }
}

fn split(v: &str) -> Vec<String> {
    v.split('/').map(|x| x.to_string()).collect::<Vec<String>>()
}

fn join(v: &[String]) -> String {
    v.join("/")
}

#[cfg(test)]
mod tests {
    use super::Action;
    use std::str::FromStr;

    #[test]
    fn test_action_from_str() {
        assert_eq!(
            Action::from_str("upt install $").unwrap(),
            Action {
                cmd: "upt".to_string(),
                subcmd: vec!["install".to_string()],
                options: vec![],
                args: vec![],
                has_pkg: true,
            }
        );
        assert_eq!(
            Action::from_str("upt search $").unwrap(),
            Action {
                cmd: "upt".to_string(),
                subcmd: vec!["search".to_string()],
                options: vec![],
                args: vec![],
                has_pkg: true,
            }
        );
        assert_eq!(
            Action::from_str("upt remove/uninstall $").unwrap(),
            Action {
                cmd: "upt".to_string(),
                subcmd: vec!["remove".to_string(), "uninstall".to_string()],
                options: vec![],
                args: vec![],
                has_pkg: true,
            }
        );
        assert_eq!(
            Action::from_str("apt list --installed").unwrap(),
            Action {
                cmd: "apt".to_string(),
                subcmd: vec!["list".to_string()],
                options: vec![vec!["--installed".to_string()]],
                args: vec![],
                has_pkg: false,
            }
        );
        assert_eq!(
            Action::from_str("pacman -R -s $").unwrap(),
            Action {
                cmd: "pacman".to_string(),
                subcmd: vec![],
                options: vec![vec!["-R".to_string()], vec!["-s".to_string()]],
                args: vec![],
                has_pkg: true,
            }
        );
        assert_eq!(
            Action::from_str("pacman -S -y -y").unwrap(),
            Action {
                cmd: "pacman".to_string(),
                subcmd: vec![],
                options: vec![
                    vec!["-S".to_string()],
                    vec!["-y".to_string()],
                    vec!["-y".to_string()]
                ],
                args: vec![],
                has_pkg: false,
            }
        );
        assert_eq!(
            Action::from_str("pacman -S $").unwrap(),
            Action {
                cmd: "pacman".to_string(),
                subcmd: vec![],
                options: vec![vec!["-S".to_string()]],
                args: vec![],
                has_pkg: true,
            }
        );
        assert_eq!(
            Action::from_str("scoop update *").unwrap(),
            Action {
                cmd: "scoop".to_string(),
                subcmd: vec!["update".to_string()],
                options: vec![],
                args: vec!["*".to_string()],
                has_pkg: false,
            }
        );
        assert_eq!(
            Action::from_str("choco upgrade all").unwrap(),
            Action {
                cmd: "choco".to_string(),
                subcmd: vec!["upgrade".to_string()],
                options: vec![],
                args: vec!["all".to_string()],
                has_pkg: false,
            }
        );
    }

    macro_rules! check_action_parse {
        ($input:expr, $confirm:expr, [$($args:expr),*], ($pkg:expr, $confirm_result:expr)) => {
            {
                let action = Action::from_str($input).unwrap();
                let args = vec![$($args.to_string()),*];
                let pkg = if $pkg.len() == 0 {
                    None
                } else {
                    Some($pkg.to_string())
                };
                assert_eq!(action.parse(&args, $confirm).unwrap(), (pkg, $confirm_result));
            }
        };
        ($input:expr, $confirm:expr, [$($args:expr),*]) => {
            {
                let action = Action::from_str($input).unwrap();
                let args = vec![ $($args.to_string()),*];
                assert_eq!(action.parse(&args, $confirm), None);
            }
        }
    }

    #[test]
    fn test_action_parse() {
        check_action_parse!(
            "apt install $",
            "-y/--confirm",
            ["apt", "install", "vim"],
            ("vim", false)
        );
        check_action_parse!(
            "apt install $",
            "-y/--confirm",
            ["apt", "install", "-y", "vim"],
            ("vim", true)
        );
        check_action_parse!(
            "apt install $",
            "-y/--confirm",
            ["apt", "install", "--confirm", "vim"],
            ("vim", true)
        );
        check_action_parse!(
            "apt install $",
            "-y/--confirm",
            ["apt", "install", "vim", "jq"],
            ("vim jq", false)
        );
        check_action_parse!("apt install $", "-y/--confirm", ["upt", "install", "vim"]);
        check_action_parse!("apt search $", "", ["apt", "search", "vim"], ("vim", false));
        check_action_parse!(
            "apt list --installed",
            "",
            ["apt", "list", "--installed"],
            ("", false)
        );
        check_action_parse!(
            "pacman -R -s $",
            "--noconfirm",
            ["pacman", "-R", "-s", "--noconfirm", "vim"],
            ("vim", true)
        );
        check_action_parse!(
            "pacman -R -s $",
            "--noconfirm",
            ["pacman", "-Rs", "vim"],
            ("vim", false)
        );
        check_action_parse!(
            "pacman -R -s $",
            "--noconfirm",
            ["pacman", "-Rs", "--noconfirm", "vim", "jq"],
            ("vim jq", true)
        );
        check_action_parse!("pacman -S -y -y", "", ["pacman", "-Syy"], ("", false));
        check_action_parse!("pacman -S $", "", ["pacman", "-S", "vim"], ("vim", false));
        check_action_parse!("apt search $", "", ["apt", "search"]);
        check_action_parse!("apt upgrade", "", ["apt", "upgrade", "vim"]);
        check_action_parse!("pacman -S -y -y", "", ["pacman", "-Sy"]);
        check_action_parse!("pacman -S -y -y", "", ["pacman", "-Syyy"]);
        check_action_parse!("pacman -Q -i", "", ["pacman", "-Qiy"]);
    }

    macro_rules! check_action_to_cmd {
        ($input:expr, ($pkg:expr, $confirm:expr), $cmd:expr) => {{
            let action = Action::from_str($input).unwrap();
            assert_eq!(action.to_cmd($pkg, $confirm), Some($cmd.to_string()));
        }};
        ($input:expr, ($pkg:expr, $confirm:expr)) => {{
            let action = Action::from_str($input).unwrap();
            assert!(action.to_cmd($pkg, $confirm).is_none());
        }};
    }

    #[test]
    fn test_action_to_cmd() {
        check_action_to_cmd!("apt install $", ("vim", ""), "apt install vim");
        check_action_to_cmd!("apt install $", ("vim", "-y"), "apt install vim -y");
        check_action_to_cmd!("apt install $", ("vim jq", ""), "apt install vim jq");
        check_action_to_cmd!("apt search $", ("vim", ""), "apt search vim");
        check_action_to_cmd!("apt list --installed", ("", ""), "apt list --installed");
        check_action_to_cmd!(
            "pacman -R -s $",
            ("vim", "--noconfirm"),
            "pacman -R -s vim --noconfirm"
        );
        check_action_to_cmd!("pacman -R -s $", ("vim", ""), "pacman -R -s vim");
        check_action_to_cmd!(
            "pacman -R -s $",
            ("vim jq", "--noconfirm"),
            "pacman -R -s vim jq --noconfirm"
        );
        check_action_to_cmd!("pacman -S -y -y", ("", ""), "pacman -S -y -y");
        check_action_to_cmd!("pacman -S $", ("vim", ""), "pacman -S vim");
        check_action_to_cmd!("scoop update *", ("", ""), "scoop update *");
        check_action_to_cmd!("choco upgrade all", ("", "-y"), "choco upgrade all -y");
    }

    macro_rules! check_action_help {
        ($input:expr, $help:expr) => {{
            let action = Action::from_str($input).unwrap();
            assert_eq!(action.help(), Some($help.to_string()));
        }};
        ($input:expr) => {{
            let action = Action::from_str($input).unwrap();
            assert!(action.help().is_none());
        }};
    }

    #[test]
    fn test_action_help() {
        check_action_help!("upt install $", "upt install <pkg>");
        check_action_help!("upt search $", "upt search <pkg>");
        check_action_help!("upt list -i/--installed", "upt list -i/--installed");
        check_action_help!("pacman -S -y -y", "pacman -S -y -y");
        check_action_help!("pacman -S $", "pacman -S <pkg>");
    }
}
