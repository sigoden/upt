vendor! {
    name: "apt",
    yes: ["-y", "--yes"],
    install: "install $",
    remove: "remove $",
    upgrade: "install $",
    search: "search $",
    show: "show $",
    update_index: "update",
    upgrade_all: "upgrade",
    list_upgradable: "list -u|--upgradable",
    list_installed: "list -i|--installed",
}