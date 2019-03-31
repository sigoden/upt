create_vendor! {
    name: "apk",
    install: "add -y|--yes@assume_yes $",
    remove: "del -y|--yes@assume_yes $",
    upgrade: "upgrade -y|--yes@assume_yes $",
    search: "search $",
    show: "show $",
    update_index: "update",
    upgrade_all: "upgrade",
    list_upgradable: "list -u|--upgradable",
    list_installed: "list -I|--installed",
}