create_vendor! {
    name: "apt",
    install: "install -y|--yes@assume_yes $",
    remove: "uninstall -y|--yes@assume_yes $",
    upgrade: "install -y|--yes@assume_yes $",
    search: "search $",
    show: "show $",
    update_index: "update",
    upgrade_all: "upgrade",
    list_upgradable: "list --upgradable",
    list_installed: "list --installed",
}