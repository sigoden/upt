#[macro_export]
macro_rules! create_vendor {
    (
        name: $name:expr,
        install: $install:expr,
        uninstall: $uninstall:expr,
        upgrade: $upgrade:expr,
        search: $search:expr,
        show: $show:expr,
        update_index: $update_index:expr,
        upgrade_all: $upgrade_all:expr,
        list_upgradable: $list_upgradable:expr,
        list_installed: $list_installed:expr,
    ) => {
        use super::{Parser, Vendor};
        use std::str::FromStr;
        pub fn init() -> Vendor {
            Vendor {
                name: $name.to_string(),
                install: Parser::from_str($install).unwrap(),
                uninstall: Parser::from_str($uninstall).unwrap(),
                upgrade: Parser::from_str($upgrade).unwrap(),
                search: Parser::from_str($search).unwrap(),
                show: Parser::from_str($show).unwrap(),
                update_index: Parser::from_str($update_index).unwrap(),
                upgrade_all: Parser::from_str($upgrade_all).unwrap(),
                list_upgradable: Parser::from_str($list_upgradable).unwrap(),
                list_installed: Parser::from_str($list_installed).unwrap(),
            }
        }
    }
}