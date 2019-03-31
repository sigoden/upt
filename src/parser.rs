use std::str::FromStr;

/// Subcommand parser
#[derive(Debug, PartialEq, Default)]
pub struct Parser {
    command: String,
    assume_yes: Vec<String>,
    have_operands: bool,
    required_options: Vec<Vec<String>>,
}

impl FromStr for Parser {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        if s == "" {
            return Ok(Parser {
                command: "-".to_string(),
                ..Default::default()
            })
        }
        let words: Vec<&str> = s.split(' ').collect();
        let (command, remind) = words.split_first().expect("no command");
        let mut have_operands = false;
        let mut required_options: Vec<Vec<String>> = vec![];
        let mut assume_yes: Vec<String> = Vec::new();
        fn split(v: &str) -> Vec<String> {
            v.split('|').map(|x| x.to_string()).collect::<Vec<String>>()
        }
        for elem in remind {
            if elem.starts_with("-") {
                if elem.ends_with("@assume_yes") {
                    assume_yes = split(&elem[0..elem.len() - 11]);
                } else {
                    required_options.push(split(elem));
                }
            } else if *elem == "$" {
                have_operands = true;
            }
        }
        Ok(Parser {
            command: command.to_string(),
            assume_yes,
            have_operands,
            required_options,
        })
    }
}


impl Parser {
    /// Try to parse the command line arguemnts
    pub fn parse(&self, args: &[String]) -> Option<(Option<String>, bool)> {
        if self.is_noop() {
            return None;
        }
        let (command, options, pkg) = self.grouping_args(args);
        if command != self.command {
            return None;
        }
        if pkg.is_none() && self.have_operands {
            return None;
        } else if pkg.is_some() && !self.have_operands {
            return None;
        }
        let assume_yes = if self.assume_yes.len() > 0 {
            options.iter().find(|v| self.assume_yes.iter().find(|x| x == v).is_some()).is_some()
        } else {
            false
        };
        if !self.satisfy_required_options(&options, assume_yes) {
            return None;
        }
        Some((pkg, assume_yes))
    }
    /// Generate corespond cmd
    pub fn generate_cmd(&self, pkg: &Option<String>, assume_yes: bool) -> String {
        if self.is_noop() {
            return String::new();
        }
        let mut output = self.command.clone();
        for ro in &self.required_options {
            output.push_str(" ");
            output.push_str(&ro[0]);
        }
        if self.assume_yes.len() > 0 && assume_yes {
            output.push_str(" ");
            output.push_str(&self.assume_yes[0]);
        }
        if let Some(pkg_value) = pkg {
            output.push_str(" ");
            output.push_str(pkg_value)
        }
        output
    }
    /// Genereate help message
    pub fn generate_help(&self) -> String {
        if self.is_noop() {
            return String::new();
        }
        let mut output = self.command.clone();
        for ro in &self.required_options {
            if ro.len() > 1 {
                output.push_str(" {");
                output.push_str(&ro.join("|"));
                output.push_str("}");
            } else {
                output.push_str(" ");
                output.push_str(&ro[0]);
            }
        }
        if self.assume_yes.len() > 0 {
            output.push_str(" [");
            output.push_str(&self.assume_yes.join("|"));
            output.push_str("]");
        }
        if self.have_operands {
            output.push_str(" ");
            output.push_str("<pkg>")
        }
        output
    }
    /// Whether parser is noop parser
    fn is_noop(&self) -> bool {
        &self.command == "-"
    }
    fn grouping_args<'a, 'b>(&'a self, args: &'b [String]) -> (String, Vec<String>, Option<String>) {
        let is_command_with_dash = self.command.starts_with("-");
        let mut command;
        let mut options: Vec<String> = vec![];
        let mut operands: Vec<String> = vec![];
        for arg in args.into_iter() {
            if arg.starts_with("-") {
                if arg.starts_with("--") {
                    options.push(arg.to_string());
                } else {
                    for i in 1..arg.len() {
                        let single_arg = "-".to_string() + &arg[i..i + 1];
                        options.push(single_arg);
                    }
                }
            } else {
                operands.push(arg.to_string());
            }
        }

        if is_command_with_dash {
            command = options.remove(0);
        } else {
            command = operands.remove(0);
        }
        let pkg = if operands.len() > 0 {
            Some(operands.join(" "))
        } else {
            None
        };
        (command, options, pkg)
    }
    fn satisfy_required_options(&self, options: &Vec<String>, assume_yes: bool) -> bool {
        let mut coll: Vec<(usize, &String)> = options.iter().enumerate().collect();
        let mut count_removing = 0;
        for ro in self.required_options.iter() {
            match coll
                .iter()
                .find(|(_, v)| ro.iter().find(|x| x == v).is_some())
            {
                None => return false,
                Some((i, _)) => {
                    coll.remove(*i - count_removing);
                    count_removing += 1;
                },
            };
        }
        if  coll.len() - (assume_yes as usize) != 0 {
            return  false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use std::str::FromStr;

    macro_rules! check_parser_from_str {
        ($input:expr, { $command:expr, [$($ay:expr),*], [$([$($ro:expr),* $(,)*]),*], $op:expr }) => {
            let parser = Parser::from_str($input).unwrap();
            let expect_parser = Parser {
                command: $command.to_string(),
                assume_yes: vec![ $($ay.to_string()),*],
                have_operands: $op,
                required_options: vec![$(vec![$($ro.to_string(),)*],)*],
            };
            assert_eq!(parser, expect_parser);
        }
    }
    #[test]
    fn test_parser_from_str() {
        check_parser_from_str!(
            "install -y|--yes@assume_yes $",
            { "install", ["-y", "--yes"], [], true }
        );
        check_parser_from_str!(
            "search $",
            { "search", [], [], true }
        );
        check_parser_from_str!(
            "update",
            { "update", [], [], false }
        );
        check_parser_from_str!(
            "list --installed",
            { "list", [], [["--installed"]], false }
        );
        check_parser_from_str!(
            "-R -s --noconfirm@assume_yes $",
            { "-R", ["--noconfirm"], [["-s"]], true }
        );
        check_parser_from_str!(
            "-S -y -y",
            { "-S", [], [["-y"], ["-y"]], false }
        );
        check_parser_from_str!(
            "-S $",
            { "-S", [], [], true }
        );
        check_parser_from_str!(
            "",
            { "-", [], [], false }
        );
    }
    macro_rules! check_parser_parse {
        ($input:expr, [$($arg:expr),*], ($pkg:expr, $assume_yes:expr)) => {
            {
                let parser = Parser::from_str($input).unwrap();
                let args = vec![ $($arg.to_string()),*];
                let pkg = if $pkg.len() == 0 {
                    None
                } else {
                    Some($pkg.to_string())
                };
                assert_eq!(parser.parse(&args).unwrap(), (pkg, $assume_yes));
            }
        };
        ($input:expr, [$($arg:expr),*]) => {
            {
                let parser = Parser::from_str($input).unwrap();
                let args = vec![ $($arg.to_string()),*];
                assert_eq!(parser.parse(&args), None);
            }
        }
    }
    #[test]
    fn test_parser_parse() {
        check_parser_parse!(
            "install -y|--yes@assume_yes $",
            ["install", "vim"],
            ("vim", false)
        );
        check_parser_parse!(
            "install -y|--yes@assume_yes $",
            ["install", "-y", "vim"],
            ("vim", true)
        );
        check_parser_parse!(
            "install -y|--yes@assume_yes $",
            ["install", "--yes", "vim"],
            ("vim", true)
        );
        check_parser_parse!(
            "install -y|--yes@assume_yes $",
            ["install", "vim", "jq"],
            ("vim jq", false)
        );
        check_parser_parse!(
            "search $",
            ["search", "vim"],
            ("vim", false)
        );
        check_parser_parse!(
            "list --installed",
            ["list", "--installed"],
            ("", false)
        );
        check_parser_parse!(
            "-R -s --noconfirm@assume_yes $",
            ["-R", "-s", "--noconfirm", "vim"],
            ("vim", true)
        );
        check_parser_parse!(
            "-R -s --noconfirm@assume_yes $",
            ["-Rs", "vim"],
            ("vim", false)
        );
        check_parser_parse!(
            "-R -s --noconfirm@assume_yes $",
            ["-Rs", "--noconfirm", "vim", "jq"],
            ("vim jq", true)
        );
        check_parser_parse!(
            "-S -y -y",
            ["-Syy"],
            ("", false)
        );
        check_parser_parse!(
            "-S $",
            ["-S", "vim"],
            ("vim", false)
        );
        check_parser_parse!(
            "search $",
            ["search"]
        );
        check_parser_parse!(
            "upgrade",
            ["upgrade", "vim"]
        );
        check_parser_parse!(
            "-S -y -y",
            ["-Sy"]
        );
        check_parser_parse!(
            "-S -y -y",
            ["-Syyy"]
        );
        check_parser_parse!(
            "-Q -i",
            ["-Qiy"]
        );
    }
    macro_rules! check_parser_generate_cmd {
        ($input:expr, ($pkg:expr, $assume_yes:expr), $cmd:expr) => {
            {
                let parser = Parser::from_str($input).unwrap();
                let pkg = if $pkg.len() == 0 {
                    None
                } else {
                    Some($pkg.to_string())
                };
                
                assert_eq!(parser.generate_cmd(&pkg, $assume_yes), $cmd.to_string());
            }
        }
    }
    #[test]
    fn test_parser_generate_cmd() {
        check_parser_generate_cmd!(
            "install -y|--yes@assume_yes $",
            ("vim", false),
            "install vim"
        );
        check_parser_generate_cmd!(
            "install -y|--yes@assume_yes $",
            ("vim", true),
            "install -y vim"
        );
        check_parser_generate_cmd!(
            "install -y|--yes@assume_yes $",
            ("vim jq", false),
            "install vim jq"
        );
        check_parser_generate_cmd!(
            "search $",
            ("vim", false),
            "search vim"
        );
        check_parser_generate_cmd!(
            "list --installed",
            ("", false),
            "list --installed"
        );
        check_parser_generate_cmd!(
            "-R -s --noconfirm@assume_yes $",
            ("vim", true),
            "-R -s --noconfirm vim"
        );
        check_parser_generate_cmd!(
            "-R -s --noconfirm@assume_yes $",
            ("vim", false),
            "-R -s vim"
        );
        check_parser_generate_cmd!(
            "-R -s --noconfirm@assume_yes $",
            ("vim jq", true),
            "-R -s --noconfirm vim jq"
        );
        check_parser_generate_cmd!(
            "-S -y -y",
            ("", false),
            "-S -y -y"
        );
        check_parser_generate_cmd!(
            "-S $",
            ("vim", false),
            "-S vim"
        );
    }
    macro_rules! check_parser_generate_help {
        ($input:expr, $help:expr) => {
            {
                let parser = Parser::from_str($input).unwrap();
                assert_eq!(parser.generate_help(), $help.to_string());
            }
        }
    }
    #[test]
    fn test_parser_generate_help() {
        check_parser_generate_help!(
            "install -y|--yes@assume_yes $",
            "install [-y|--yes] <pkg>"
        );
        check_parser_generate_help!(
            "search $",
            "search <pkg>"
        );
        check_parser_generate_help!(
            "list -i|--installed",
            "list {-i|--installed}"
        );
        check_parser_generate_help!(
            "-R -s --noconfirm@assume_yes $",
            "-R -s [--noconfirm] <pkg>"
        );
        check_parser_generate_help!(
            "-S -y -y",
            "-S -y -y"
        );
        check_parser_generate_help!(
            "-S $",
            "-S <pkg>"
        );
    }
}