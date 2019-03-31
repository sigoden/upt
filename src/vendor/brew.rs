create_vendor! {
    name: "brew",
    install: "install -y|--yes@assume_yes $",
    remove: "uninstall -y|--yes@assume_yes $",
    upgrade: "upgrade -y|--yes@assume_yes $",
    search: "search $",
    show: "info $",
    update_index: "update",
    upgrade_all: "upgrade",
    list_upgradable: "outdated",
    list_installed: "list",
}
