macro_rules! vendors {
    (
        $(
            {
                name: $name:literal,
                confirm: $confirm:literal,
                install: $install:literal,
                remove: $remove:literal,
                upgrade: $upgrade:literal,
                search: $search:literal,
                info: $show:literal,
                update_index: $update_index:literal,
                upgrade_all: $upgrade_all:literal,
                list_upgradable: $list_upgradable:literal,
                list_installed: $list_installed:literal,
            },
        )+
    ) => {
        pub fn init(name: &str) -> Result<$crate::Vendor, $crate::UptError> {
            use $crate::subcommand::must_from_str;
            match name {
                $(
                    $name => {
                        let vendor = $crate::Vendor {
                            name: $name.to_string(),
                            confirm: $confirm.to_string(),
                            install: must_from_str($install, $name, "install"),
                            remove: must_from_str($remove, $name, "remove"),
                            upgrade: must_from_str($upgrade, $name, "upgrade"),
                            search: must_from_str($search, $name, "search"),
                            info: must_from_str($show, $name, "show"),
                            update_index: must_from_str($update_index, $name, "update_index"),
                            upgrade_all: must_from_str($upgrade_all, $name, "upgrade_all"),
                            list_upgradable: must_from_str($list_upgradable, $name, "list_upgradable"),
                            list_installed: must_from_str($list_installed, $name, "list_installed"),
                        };
                        Ok(vendor)
                    },
                )+
                _ => Err(UptError::NoVendor(name.to_string()))
            }
        }

        pub(crate) fn which_cmd(name: &str) -> Option<&'static str> {
            match name {
                $(
                    $name => {
                        let (cmd, _) = $install.split_once(' ')?;
                        Some(cmd)
                    }
                )+
                _ => None
            }
        }
    }
}

macro_rules! tools {
    ($($os:literal => $($tool:literal),+);+$(;)?) => {
        pub fn detect_tool() -> std::result::Result<$crate::Vendor, $crate::UptError> {
            let os = crate::utils::detect_os().ok_or(UptError::NotSupportOS)?;
            let tools: Vec<&str> = match os.as_str() {
                $(
                    $os => vec![$($tool),+].into_iter().filter_map(|v| which_cmd(v)).collect(),
                )+
                _ => vec!["apt", "dnf", "pacman"],
            };
            match $crate::utils::find_tool(&tools) {
                Some(tool) => $crate::vendor::init(&tool),
                None => Err(UptError::NotFoundTool),
            }
        }
    };
}
