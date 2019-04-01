vendor! {
    name: "pacman",
    yes: ["--noconfirm"],
    install: "-S $",
    remove: "-R -s $",
    upgrade: "-S $",
    search: "-S -s $",
    show: "-S -i $",
    update_index: "-S -y -y",
    upgrade_all: "-S -y -u",
    list_upgradable: "-Q -u",
    list_installed: "-Q -e",
}

 