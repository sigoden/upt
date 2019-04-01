vendor! {
    name: "yum",
    yes: ["-y", "--assumeyes"],
    install: "install $",
    remove: "remove $",
    upgrade: "update $",
    search: "search $",
    show: "info $",
    update_index: "check-update",
    upgrade_all: "update",
    list_upgradable: "list updates",
    list_installed: "list installed",
}
