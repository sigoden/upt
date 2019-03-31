create_vendor! {
    name: "apt",
    install: "install -y|--yes@assume_yes $",
    uninstall: "uninstall -y|--yes@assume_yes $",
    upgrade: "",
    search: "",
    show: "",
    update_index: "",
    upgrade_all: "",
    list_upgradable: "",
    list_installed: "",
}