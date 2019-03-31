create_vendor! {
    name: "yum",
    install: "install -y|--assumeyes@assume_yes $",
    remove: "remove -y|--assumeyes@assume_yes $",
    upgrade: "update -y|--assumeyes@assume_yes $",
    search: "search $",
    show: "info $",
    update_index: "check-update",
    upgrade_all: "update",
    list_upgradable: "list updates",
    list_installed: "list installed",
}