macro_rules! vendors {
    (
        $(
            $key:ident: {
                name: $name:expr,
                yes: [$($yes:expr),*],
                install: $install:expr,
                remove: $remove:expr,
                upgrade: $upgrade:expr,
                search: $search:expr,
                show: $show:expr,
                update_index: $update_index:expr,
                upgrade_all: $upgrade_all:expr,
                list_upgradable: $list_upgradable:expr,
                list_installed: $list_installed:expr,
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
                            yes: vec![ $($yes.to_string()),* ],
                            install: must_from_str($install, $name, "install"),
                            remove: must_from_str($remove, $name, "remove"),
                            upgrade: must_from_str($upgrade, $name, "upgrade"),
                            search: must_from_str($search, $name, "search"),
                            show: must_from_str($show, $name, "show"),
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
    };
}

macro_rules! detect_tool {
    ($( $os:literal => ($($vendor:literal),+ $(,)?), )+) => {
        pub fn detect_tool() -> std::result::Result<$crate::Vendor, $crate::UptError> {
            let os = crate::utils::detect_os().ok_or(UptError::NotSupportOS)?;
            let tools: Vec<&str> = match os.as_str() {
                $(
                    $os => vec![$($vendor),+],
                )+
                _ => return Err(UptError::NotSupportOS),
            };
            match $crate::utils::find_tool(&tools) {
                Some(tool) => $crate::vendor::init(&tool),
                None => Err(UptError::NotFoundTool),
            }
        }
    };
}
