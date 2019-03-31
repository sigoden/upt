create_vendor! {
    name: "choco",
    install: "install -y@assume_yes $",
    remove: "uninstall -y@assume_yes $",
    upgrade: "upgrade -y@assume_yes $",
    search: "search $",
    show: "info $",
    update_index: "upgrade --noop",
    upgrade_all: "upgrade all",
    list_upgradable: "outdated",
    list_installed: "list -lai",
}
