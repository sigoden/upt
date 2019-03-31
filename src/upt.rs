use crate::{Vender};
use crate::parser::Parser;
pub struct Upt {
    name: String,
    install: Parser,
    uninstall: Parser,
    upgrade: Parser,
    search: Parser,
    show: Parser,
    upgrade_index: Parser,
    update_all: Parser,
    list_upgradable: Parser,
    list_installed: Parser,
}

impl Vender for Upt {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn parser_install(&self) -> &Parser {
        &self.install
    }
    fn parser_uninstall(&self) -> &Parser {
        &self.uninstall
    }
    fn parser_upgrade(&self) -> &Parser {
        &self.upgrade
    }
    fn parser_search(&self) -> &Parser {
        &self.search
    }
    fn parser_show(&self) -> &Parser {
        &self.show
    }
    fn parser_update_index(&self) -> &Parser {
        &self.upgrade_index
    }
    fn parser_upgrade_all(&self) -> &Parser {
        &self.update_all
    }
    fn parser_list_upgradable(&self) -> &Parser {
        &self.list_upgradable
    }
    fn parser_list_installed(&self) -> &Parser {
        &self.list_installed
    }
}