vendor! {
    name: "dnf",
    yes: ["-y", "--assumeyes"],
    install: "install $",
    remove: "remove $",
    upgrade: "upgrade $",
    search: "search $",
    show: "info $",
    update_index: "check-update",
    upgrade_all: "update",
    list_upgradable: "list --upgrades",
    list_installed: "list --installed",
}
