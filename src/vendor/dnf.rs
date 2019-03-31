create_vendor! {
    name: "dnf",
    install: "install -y|--assumeyes@assume_yes $",
    remove: "remove -y|--assumeyes@assume_yes $",
    upgrade: "upgrade -y|--assumeyes@assume_yes $",
    search: "search $",
    show: "info $",
    update_index: "check-update",
    upgrade_all: "update",
    list_upgradable: "list --upgrades",
    list_installed: "list --installed",
}
