use crate::UptError;

use std::cell::Cell;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct SubCommand {
    cmd: String,
    action: Option<String>,
    has_pkg: bool,
    options: Vec<Vec<String>>,
}

impl FromStr for SubCommand {
    type Err = UptError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Default::default());
        }
        let words: Vec<&str> = s.split(' ').collect();
        let mut has_pkg = false;
        let mut options: Vec<Vec<String>> = vec![];
        if words.len() < 2 {
            return Err(UptError::InvalidSubcommand(s.to_string()));
        }
        let (cmd, action, reminder) = if words[1].starts_with('-') || words[1] == "$" {
            (words[0].to_string(), None, &words[1..])
        } else {
            (
                words[0].to_string(),
                Some(words[1].to_string()),
                &words[2..],
            )
        };
        fn split(v: &str) -> Vec<String> {
            v.split('/').map(|x| x.to_string()).collect::<Vec<String>>()
        }
        for elem in reminder {
            if elem == &"$" {
                has_pkg = true;
                continue;
            }
            if elem.starts_with('-') {
                options.push(split(elem));
            }
        }
        Ok(SubCommand {
            cmd,
            action,
            has_pkg,
            options,
        })
    }
}

impl SubCommand {
    /// Try to parse the command line arguemnts
    pub fn parse(&self, args: &[String], confirm: &str) -> Option<(Option<String>, bool)> {
        if self.is_default() {
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
        if self.is_default() {
            return None;
        }
        let mut segs: Vec<&str> = vec![&self.cmd];
        if let Some(action) = &self.action {
            segs.push(action);
        }
        for item in &self.options {
            segs.push(&item[0]);
        }
        if !confirm.is_empty() {
            segs.push(confirm);
        }
        if !pkg.is_empty() {
            segs.push(pkg);
        }
        Some(segs.join(" "))
    }

    /// Genereate help message
    pub fn help(&self) -> Option<String> {
        if self.is_default() {
            return None;
        }
        let mut segs: Vec<String> = vec![self.cmd.clone()];

        if let Some(action) = &self.action {
            segs.push(action.clone());
        }

        for item in &self.options {
            if item.len() > 1 {
                segs.push(item.join("/"));
            } else {
                segs.push(item[0].clone());
            }
        }
        if self.has_pkg {
            segs.push(String::from("<pkg>"));
        }
        Some(segs.join(" "))
    }

    fn is_default(&self) -> bool {
        self == &Default::default()
    }

    fn parse_args(&self, args: &[String]) -> Option<(Vec<String>, Option<String>)> {
        if args.len() < 2 {
            return None;
        }
        if self.cmd != args[0] {
            return None;
        }
        let reminder = if let Some(action) = &self.action {
            if &args[1] != action {
                return None;
            }
            &args[2..]
        } else {
            &args[1..]
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
pub(crate) fn must_from_str(s: &str, name: &str, field: &str) -> SubCommand {
    match SubCommand::from_str(s) {
        Ok(p) => p,
        Err(_) => panic!("Failed to parse {}.{} from '{}' ", name, field, s),
    }
}

#[cfg(test)]
mod tests {
    use super::SubCommand;
    use std::str::FromStr;

    macro_rules! check_subcommand_from_str {
        ($input:expr, { $cmd:expr, $action:expr, [$([$($options:expr),* $(,)*]),*], $has_pkg:expr }) => {
            let subcommand = SubCommand::from_str($input).unwrap();
            let expect_subcommand = SubCommand {
                cmd: $cmd.to_string(),
                action: $action.map(|v| v.to_string()),
                has_pkg: $has_pkg,
                options: vec![$(vec![$($options.to_string(),)*],)*],
            };
            assert_eq!(subcommand, expect_subcommand);
        }
    }

    #[test]
    fn test_subcommand_from_str() {
        check_subcommand_from_str!(
            "upt install $",
            { "upt", Some("install"), [], true }
        );
        check_subcommand_from_str!(
            "upt search $",
            { "upt", Some("search"), [], true }
        );
        check_subcommand_from_str!(
            "apt list --installed",
            {"apt", Some("list"), [["--installed"]], false }
        );
        check_subcommand_from_str!(
            "pacman -R -s $",
            { "pacman", None::<&str>, [["-R"], ["-s"]], true }
        );
        check_subcommand_from_str!(
            "pacman -S -y -y",
            { "pacman", None::<&str>, [["-S"], ["-y"], ["-y"]], false }
        );
        check_subcommand_from_str!(
            "pacman -S $",
            { "pacman", None::<&str>, [["-S"]], true }
        );
    }

    macro_rules! check_subcommand_parse {
        ($input:expr, $confirm:expr, [$($args:expr),*], ($pkg:expr, $confirm_result:expr)) => {
            {
                let subcommand = SubCommand::from_str($input).unwrap();
                let args = vec![$($args.to_string()),*];
                let pkg = if $pkg.len() == 0 {
                    None
                } else {
                    Some($pkg.to_string())
                };
                assert_eq!(subcommand.parse(&args, $confirm).unwrap(), (pkg, $confirm_result));
            }
        };
        ($input:expr, $confirm:expr, [$($args:expr),*]) => {
            {
                let subcommand = SubCommand::from_str($input).unwrap();
                let args = vec![ $($args.to_string()),*];
                assert_eq!(subcommand.parse(&args, $confirm), None);
            }
        }
    }

    #[test]
    fn test_subcommand_parse() {
        check_subcommand_parse!(
            "apt install $",
            "-y/--confirm",
            ["apt", "install", "vim"],
            ("vim", false)
        );
        check_subcommand_parse!(
            "apt install $",
            "-y/--confirm",
            ["apt", "install", "-y", "vim"],
            ("vim", true)
        );
        check_subcommand_parse!(
            "apt install $",
            "-y/--confirm",
            ["apt", "install", "--confirm", "vim"],
            ("vim", true)
        );
        check_subcommand_parse!(
            "apt install $",
            "-y/--confirm",
            ["apt", "install", "vim", "jq"],
            ("vim jq", false)
        );
        check_subcommand_parse!("apt install $", "-y/--confirm", ["upt", "install", "vim"]);
        check_subcommand_parse!("apt search $", "", ["apt", "search", "vim"], ("vim", false));
        check_subcommand_parse!(
            "apt list --installed",
            "",
            ["apt", "list", "--installed"],
            ("", false)
        );
        check_subcommand_parse!(
            "pacman -R -s $",
            "--noconfirm",
            ["pacman", "-R", "-s", "--noconfirm", "vim"],
            ("vim", true)
        );
        check_subcommand_parse!(
            "pacman -R -s $",
            "--noconfirm",
            ["pacman", "-Rs", "vim"],
            ("vim", false)
        );
        check_subcommand_parse!(
            "pacman -R -s $",
            "--noconfirm",
            ["pacman", "-Rs", "--noconfirm", "vim", "jq"],
            ("vim jq", true)
        );
        check_subcommand_parse!("pacman -S -y -y", "", ["pacman", "-Syy"], ("", false));
        check_subcommand_parse!("pacman -S $", "", ["pacman", "-S", "vim"], ("vim", false));
        check_subcommand_parse!("apt tsearch $", "", ["apt", "search"]);
        check_subcommand_parse!("apt tupgrade", "", ["apt", "upgrade", "vim"]);
        check_subcommand_parse!("pacman -S -y -y", "", ["pacman", "-Sy"]);
        check_subcommand_parse!("pacman -S -y -y", "", ["pacman", "-Syyy"]);
        check_subcommand_parse!("pacman -Q -i", "", ["pacman", "-Qiy"]);
    }

    macro_rules! check_subcommand_to_cmd {
        ($input:expr, ($pkg:expr, $confirm:expr), $cmd:expr) => {{
            let subcommand = SubCommand::from_str($input).unwrap();
            assert_eq!(subcommand.to_cmd($pkg, $confirm), Some($cmd.to_string()));
        }};
        ($input:expr, ($pkg:expr, $confirm:expr)) => {{
            let subcommand = SubCommand::from_str($input).unwrap();
            assert!(subcommand.to_cmd($pkg, $confirm).is_none());
        }};
    }

    #[test]
    fn test_subcommand_to_cmd() {
        check_subcommand_to_cmd!("apt install $", ("vim", ""), "apt install vim");
        check_subcommand_to_cmd!("apt install $", ("vim", "-y"), "apt install -y vim");
        check_subcommand_to_cmd!("apt install $", ("vim jq", ""), "apt install vim jq");
        check_subcommand_to_cmd!("apt search $", ("vim", ""), "apt search vim");
        check_subcommand_to_cmd!("apt list --installed", ("", ""), "apt list --installed");
        check_subcommand_to_cmd!(
            "pacman -R -s $",
            ("vim", "--noconfirm"),
            "pacman -R -s --noconfirm vim"
        );
        check_subcommand_to_cmd!("pacman -R -s $", ("vim", ""), "pacman -R -s vim");
        check_subcommand_to_cmd!(
            "pacman -R -s $",
            ("vim jq", "--noconfirm"),
            "pacman -R -s --noconfirm vim jq"
        );
        check_subcommand_to_cmd!("pacman -S -y -y", ("", ""), "pacman -S -y -y");
        check_subcommand_to_cmd!("pacman -S $", ("vim", ""), "pacman -S vim");
    }

    macro_rules! check_subcommand_help {
        ($input:expr, $help:expr) => {{
            let subcommand = SubCommand::from_str($input).unwrap();
            assert_eq!(subcommand.help(), Some($help.to_string()));
        }};
        ($input:expr) => {{
            let subcommand = SubCommand::from_str($input).unwrap();
            assert!(subcommand.help().is_none());
        }};
    }

    #[test]
    fn test_subcommand_help() {
        check_subcommand_help!("upt install $", "upt install <pkg>");
        check_subcommand_help!("upt search $", "upt search <pkg>");
        check_subcommand_help!("upt list -i/--installed", "upt list -i/--installed");
        check_subcommand_help!("pacman -S -y -y", "pacman -S -y -y");
        check_subcommand_help!("pacman -S $", "pacman -S <pkg>");
    }
}
