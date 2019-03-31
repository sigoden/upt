create_vendor! {
    name: "pacman",
    install: "-S --noconfirm@assume_yes $",
    remove: "-R -s --noconfirm@assume_yes $",
    upgrade: "-S --noconfirm@assume_yes $",
    search: "-S -s $",
    show: "-S -i $",
    update_index: "-S -y -y",
    upgrade_all: "-S -y -u",
    list_upgradable: "-Q -u",
    list_installed: "-Q -e",
}

 