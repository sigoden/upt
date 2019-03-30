use std::str::FromStr;

/// Command line parser
#[derive(Debug, PartialEq)]
pub struct Parser {
    command: String,
    assume_yes: Vec<String>,
    have_operands: bool,
    required_options: Vec<Vec<String>>,
}

impl FromStr for Parser {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let words: Vec<&str> = s.split(' ').collect();
        let (command, remind) = words.split_first().expect("no command");
        let mut have_operands = false;
        let mut required_options: Vec<Vec<String>> = vec![];
        let mut assume_yes: Vec<String> = Vec::new();
        fn split(v: &str) -> Vec<String> {
            v.split('|').map(|x| x.trim_start_matches('-').to_string()).collect::<Vec<String>>()
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
        // unimplemented!()
        Ok(Parser {
            command: command.to_string(),
            assume_yes,
            have_operands,
            required_options,
        })
    }
}


impl Parser {
    pub fn parse(&self, args: &[String]) -> Option<(Option<String>, bool)> {
        let (command, options, pkg) = self.grouping_args(args);
        if command != self.command {
            return None;
        }
        if pkg.is_none() && self.have_operands {
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
        Some((pkg, assume_yes))
    }
    pub fn stringify(&self, pkg: &Option<String>, assume_yes: bool) -> String {
        unimplemented!()
    }
    fn grouping_args<'a, 'b>(&'a self, args: &'b [String]) -> (String, Vec<&'b str>, Option<String>) {
        let is_command_with_dash = self.command.starts_with("-");
        let mut command;
        let mut options: Vec<&str> = vec![];
        let mut operands: Vec<&str> = vec![];
        for arg in args.into_iter() {
            if arg.starts_with("-") {
                if arg.starts_with("--") {
                    options.push(&arg[2..]);
                } else {
                    for i in 1..arg.len() {
                        options.push(&arg[i..i + 1]);
                    }
                }
            } else {
                operands.push(arg);
            }
        }

        if is_command_with_dash {
            command = options.remove(0).to_string();
            if command.len() == 1 {
                command = "-".to_string() + &command;
            } else {
                command = "--".to_string() + &command;
            }
        } else {
            command = operands.remove(0).to_string();
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
            { "install", ["y", "yes"], [], true }
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
            { "list", [], [["installed"]], false }
        );
        check_parser_from_str!(
            "-R -s --noconfirm@assume_yes $",
            { "-R", ["noconfirm"], [["s"]], true }
        );
        check_parser_from_str!(
            "-S -y -y",
            { "-S", [], [["y"], ["y"]], false }
        );
        check_parser_from_str!(
            "-S $",
            { "-S", [], [], true }
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
            "-S vim",
            ["-S", "vim"],
            ("vim", false)
        );
    }
}