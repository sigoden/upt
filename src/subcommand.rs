use std::cell::Cell;
use std::str::FromStr;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct SubCommand {
    name: String,
    has_pkg: bool,
    options: Vec<Vec<String>>,
}

impl FromStr for SubCommand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        if s == "" {
            return Ok(Default::default());
        }
        let words: Vec<&str> = s.split(' ').collect();
        let (name, remainer) = words.split_first().unwrap();
        let mut has_pkg = false;
        let mut options: Vec<Vec<String>> = vec![];
        // "-i|--installed" => ["i", "installed"]
        fn split(v: &str) -> Vec<String> {
            v.split('|').map(|x| x.to_string()).collect::<Vec<String>>()
        }
        for elem in remainer {
            if elem == &"$" {
                has_pkg = true;
                continue;
            }
            if elem.starts_with('-') {
                options.push(split(elem));
            }
        }
        Ok(SubCommand {
            name: name.to_string(),
            has_pkg,
            options,
        })
    }
}

impl SubCommand {
    /// Try to parse the command line arguemnts
    pub fn parse(&self, args: &[String], yes_options: &[String]) -> Option<(Option<String>, bool)> {
        if self.is_default() {
            return None;
        }
        let (name, options, pkg) = self.classify_args(args);
        if name != self.name {
            return None;
        }
        if (pkg.is_none() && self.has_pkg) || (pkg.is_some() && !self.has_pkg) {
            return None;
        }
        let options_no_yes: Vec<String> = options
            .iter()
            .filter(|v| yes_options.iter().find(|y| y == v).is_none())
            .cloned()
            .collect();

        let yes = options_no_yes.len() != options.len();
        if !self.satisfy_options(&options_no_yes) {
            return None;
        }
        Some((pkg, yes))
    }
    pub fn to_cmd(&self, pkg: &str, yes: &str) -> Option<String> {
        if self.is_default() {
            return None;
        }
        let mut segs: Vec<&str> = Vec::new();
        segs.push(&self.name);
        for ro in &self.options {
            segs.push(&ro[0]);
        }
        if !yes.is_empty() {
            segs.push(&yes);
        }
        if !pkg.is_empty() {
            segs.push(&pkg);
        }
        Some(segs.join(" "))
    }
    /// Genereate help message
    pub fn help(&self) -> Option<String> {
        if self.is_default() {
            return None;
        }
        let mut segs: Vec<String> = Vec::new();
        segs.push(self.name.clone());
        for ro in &self.options {
            if ro.len() > 1 {
                segs.push(["{", &ro.join("|"), "}"].concat());
            } else {
                segs.push(ro[0].clone());
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
    fn classify_args<'a, 'b>(
        &'a self,
        args: &'b [String],
    ) -> (String, Vec<String>, Option<String>) {
        let is_dashed = self.name.starts_with('-'); // SubCommand of pacman is dashed
        let mut options: Vec<String> = vec![];
        let mut operands: Vec<String> = vec![];
        for arg in args.iter() {
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
        let name = match is_dashed {
            true => {
                if options.len() == 0 {
                    String::new()
                } else {
                    options.remove(0)
                }
            }
            false => {
                if operands.len() == 0 {
                    String::new()
                } else {
                    operands.remove(0)
                }
            }
        };
        let pkg = if operands.is_empty() {
            None
        } else {
            Some(operands.join(" "))
        };
        (name, options, pkg)
    }
    fn satisfy_options(&self, options: &[String]) -> bool {
        let marks: Vec<Cell<bool>> = self.options.iter().map(|_| Cell::new(false)).collect();
        if options.len() != self.options.len() {
            return false;
        }
        options.iter().all(|v| {
            self.options
                .iter()
                .enumerate()
                .find(|(i, y)| {
                    let mark = marks.get(*i).unwrap();
                    if mark.get() == false && y.iter().find(|z| *z == v).is_some() {
                        mark.set(true);
                        return true;
                    }
                    false
                })
                .is_some()
        })
    }
}

/// used in vendor!
pub(crate) fn must_from_str(s: &str, name: &str, field: &str) -> SubCommand {
    match SubCommand::from_str(s) {
        Ok(p) => p,
        Err(_) => panic!("{}.{}: invalid subcommand '{}'", name, field, s),
    }
}

#[cfg(test)]
mod tests {
    use super::SubCommand;
    use std::str::FromStr;

    macro_rules! check_subcommand_from_str {
        ($input:expr, { $name:expr, [$([$($ro:expr),* $(,)*]),*], $op:expr }) => {
            let subcommand = SubCommand::from_str($input).unwrap();
            let expect_subcommand = SubCommand {
                name: $name.to_string(),
                has_pkg: $op,
                options: vec![$(vec![$($ro.to_string(),)*],)*],
            };
            assert_eq!(subcommand, expect_subcommand);
        }
    }
    #[test]
    fn test_subcommand_from_str() {
        check_subcommand_from_str!(
            "install $",
            { "install", [], true }
        );
        check_subcommand_from_str!(
            "search $",
            { "search", [], true }
        );
        check_subcommand_from_str!(
            "list --installed",
            { "list", [["--installed"]], false }
        );
        check_subcommand_from_str!(
            "-R -s $",
            { "-R", [["-s"]], true }
        );
        check_subcommand_from_str!(
            "-S -y -y",
            { "-S", [["-y"], ["-y"]], false }
        );
        check_subcommand_from_str!(
            "-S $",
            { "-S", [], true }
        );
    }
    macro_rules! check_subcommand_parse {
        ($input:expr, [$($yess:expr),*], [$($arg:expr),*], ($pkg:expr, $yes:expr)) => {
            {
                let subcommand = SubCommand::from_str($input).unwrap();
                let args = vec![$($arg.to_string()),*];
                let pkg = if $pkg.len() == 0 {
                    None
                } else {
                    Some($pkg.to_string())
                };
                let yes = vec![$($yess.to_string()),*];
                assert_eq!(subcommand.parse(&args, &yes).unwrap(), (pkg, $yes));
            }
        };
        ($input:expr, [$($yes:expr),*], [$($arg:expr),*]) => {
            {
                let subcommand = SubCommand::from_str($input).unwrap();
                let args = vec![ $($arg.to_string()),*];
                let yes = vec![$($yes.to_string()),*];
                assert_eq!(subcommand.parse(&args, &yes), None);
            }
        }
    }
    #[test]
    fn test_subcommand_parse() {
        check_subcommand_parse!(
            "install $",
            ["-y", "--yes"],
            ["install", "vim"],
            ("vim", false)
        );
        check_subcommand_parse!(
            "install $",
            ["-y", "--yes"],
            ["install", "-y", "vim"],
            ("vim", true)
        );
        check_subcommand_parse!(
            "install $",
            ["-y", "--yes"],
            ["install", "--yes", "vim"],
            ("vim", true)
        );
        check_subcommand_parse!(
            "install $",
            ["-y", "--yes"],
            ["install", "vim", "jq"],
            ("vim jq", false)
        );
        check_subcommand_parse!("search $", [], ["search", "vim"], ("vim", false));
        check_subcommand_parse!("list --installed", [], ["list", "--installed"], ("", false));
        check_subcommand_parse!(
            "-R -s $",
            ["--noconfirm"],
            ["-R", "-s", "--noconfirm", "vim"],
            ("vim", true)
        );
        check_subcommand_parse!("-R -s $", ["--noconfirm"], ["-Rs", "vim"], ("vim", false));
        check_subcommand_parse!(
            "-R -s $",
            ["--noconfirm"],
            ["-Rs", "--noconfirm", "vim", "jq"],
            ("vim jq", true)
        );
        check_subcommand_parse!("-S -y -y", [], ["-Syy"], ("", false));
        check_subcommand_parse!("-S $", [], ["-S", "vim"], ("vim", false));
        check_subcommand_parse!("search $", [], ["search"]);
        check_subcommand_parse!("upgrade", [], ["upgrade", "vim"]);
        check_subcommand_parse!("-S -y -y", [], ["-Sy"]);
        check_subcommand_parse!("-S -y -y", [], ["-Syyy"]);
        check_subcommand_parse!("-Q -i", [], ["-Qiy"]);
    }
    macro_rules! check_subcommand_to_cmd {
        ($input:expr, ($pkg:expr, $yes:expr), $cmd:expr) => {{
            let subcommand = SubCommand::from_str($input).unwrap();
            assert_eq!(subcommand.to_cmd($pkg, $yes), Some($cmd.to_string()));
        }};
        ($input:expr, ($pkg:expr, $yes:expr)) => {{
            let subcommand = SubCommand::from_str($input).unwrap();
            assert!(subcommand.to_cmd($pkg, $yes).is_none());
        }};
    }
    #[test]
    fn test_subcommand_to_cmd() {
        check_subcommand_to_cmd!("install $", ("vim", ""), "install vim");
        check_subcommand_to_cmd!("install $", ("vim", "-y"), "install -y vim");
        check_subcommand_to_cmd!("install $", ("vim jq", ""), "install vim jq");
        check_subcommand_to_cmd!("search $", ("vim", ""), "search vim");
        check_subcommand_to_cmd!("list --installed", ("", ""), "list --installed");
        check_subcommand_to_cmd!("-R -s $", ("vim", "--noconfirm"), "-R -s --noconfirm vim");
        check_subcommand_to_cmd!("-R -s $", ("vim", ""), "-R -s vim");
        check_subcommand_to_cmd!(
            "-R -s $",
            ("vim jq", "--noconfirm"),
            "-R -s --noconfirm vim jq"
        );
        check_subcommand_to_cmd!("-S -y -y", ("", ""), "-S -y -y");
        check_subcommand_to_cmd!("-S $", ("vim", ""), "-S vim");
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
        check_subcommand_help!("install $", "install <pkg>");
        check_subcommand_help!("search $", "search <pkg>");
        check_subcommand_help!("list -i|--installed", "list {-i|--installed}");
        check_subcommand_help!("-S -y -y", "-S -y -y");
        check_subcommand_help!("-S $", "-S <pkg>");
    }
}
