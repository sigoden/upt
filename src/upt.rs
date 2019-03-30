// macro_rules! foo {
//     () => {}
// }

// use std::str::FromStr;

// foo!(upt => [
//     Install: "install -y? $", // [Subcommand<install>, ]
//     Uninstall: "remove -y? $",
//     Upgrade: "upgrade -y? $",
//     Search: "search $",
//     Show: "search $",
//     UpdateIndex: "update",
//     UpgradeAll: "upgrade",
//     ListUpgradable: "list --upgradable",
//     ListInstalled: "list --installed",
// ]);

// foo!(pacman => [
//     Uninstall: "-R -s --noconfirm? $",
//     Search: "-Q -s --noconfirm? $",
//     Show: "-Q -i --noconfirm? $",
//     UpdateIndex: "-S -y -y",
//     UpgradeAll: "-S -y -u",
//     ListUpgradable: "-Q -u",
//     ListInstalled: "-Q -e",
//     Install: "-S $",
//     Upgrade: "-S $",
// ]);

