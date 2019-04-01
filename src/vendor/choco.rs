vendor! {
    name: "choco",
    yes: ["-y"],
    install: "install $",
    remove: "uninstall $",
    upgrade: "upgrade $",
    search: "search $",
    show: "info $",
    update_index: "upgrade --noop",
    upgrade_all: "upgrade all",
    list_upgradable: "outdated",
    list_installed: "list -lai",
}
