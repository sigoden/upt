# Upt — **U**niversal **P**ackage-management **T**ool.

Upt provides a unified command interface to manage packages for all OSs. 

Upt relies on platform's package management tool to do the job, it is more like an alias.

[![Build status](https://github.com/sigoden/upt/actions/workflows/ci.yml/badge.svg)](https://github.com/sigoden/upt/actions)
[![Crates.io](https://img.shields.io/crates/v/upt.svg)](https://crates.io/crates/upt)

## Features

### Unified command interface

Each operating system (OS) has its own package management tool, which requires different commands to complete the same operation.
This can be inconvenient when switching between or trying new OSs. 

```sh
apt remove $pkg           # Ubuntu, Debian, Linux Mint...
brew uninstall $pkg       # MacOS Homebrew
choco uninstall $pkg      # Windows Chocolatey
dnf remove $pkg           # Fedora, Redhat...
nix-env -e $pkg           # Nixos
pacman -Rs $pkg           # Arch, Manjaro...
xbps-remove $pkg          # Voidlinux
```

With `upt`, You just need to remember one command:

```sh
upt remove $pkg        # Works on any OS
```

### Act as other command

Upt can act as other commands and use their syntax by renaming.

```sh
cp upt brew
brew install vim          # use brew syntax to install a package

cp upt pacman
pacman -S vim             # use pacman syntax to install a package
```

### Supported Tools

```
| Tool     | Install                      | Uninstall                    | Upgrade                          | Search                     | Info                            | Update Index             | Upgrade All              | List Installed                    |
| -------- | ---------------------------- | ---------------------------- | -------------------------------- | -------------------------- | ------------------------------- | ------------------------ | ------------------------ | --------------------------------- |
| apk      | apk add <pkg>                | apk del <pkg>                | apk upgrade <pkg>                | apk search <pkg>           | apk info <pkg>                  | apk update               | apk upgrade              | apk list -I/--installed           |
| apt      | apt install <pkg>            | apt remove <pkg>             | apt install --only-upgrade <pkg> | apt search <pkg>           | apt show <pkg>                  | apt update               | apt upgrade              | apt list -i/--installed           |
| brew     | brew install <pkg>           | brew uninstall <pkg>         | brew upgrade <pkg>               | brew search <pkg>          | brew info <pkg>                 | brew update              | brew upgrade             | brew list                         |
| cards    | cards install <pkg>          | cards remove <pkg>           | cards install -u/--upgrade <pkg> | cards search <pkg>         | cards info <pkg>                | cards sync               | cards upgrade            | cards list                        |
| choco    | choco install <pkg>          | choco uninstall <pkg>        | choco upgrade <pkg>              | choco search <pkg>         | choco info <pkg>                | choco upgrade all --noop | choco upgrade all        | choco list -l/--local-only        |
| dnf      | dnf install <pkg>            | dnf remove <pkg>             | dnf upgrade <pkg>                | dnf search <pkg>           | dnf info <pkg>                  | dnf check-update         | dnf update               | dnf list --installed              |
| emerge   | emerge <pkg>                 | emerge --deselect <pkg>      | emerge --update <pkg>            | emerge --search <pkg>      | emerge --info <pkg>             | emerge --sync            | emerge -vuDN @world      | qlist -lv                         |
| eopkg    | eopkg install <pkg>          | eopkg remove <pkg>           | eopkg upgrade <pkg>              | eopkg search <pkg>         | eopkg info <pkg>                | eopkg update-repo        | eopkg upgrade            | eopkg list-installed              |
| flatpak  | flatpak install <pkg>        | flatpak uninstall <pkg>      | flatpak update <pkg>             | flatpak search <pkg>       | flatpak info <pkg>              | -                        | flatpak update           | flatpak list                      |
| guix     | guix install <pkg>           | guix remove <pkg>            | guix upgrade <pkg>               | guix search <pkg>          | guix show <pkg>                 | guix refresh             | guix upgrade             | guix package -I/--list-installed  |
| nix-env  | nix-env -i/--install <pkg>   | nix-env -e/--uninstall <pkg> | nix-env -u/--upgrade <pkg>       | nix-env -qaP <pkg>         | nix-env -qa --description <pkg> | nix-channel --update     | nix-env -u/--upgrade     | nix-env -q/--query --installed    |
| opkg     | opkg install <pkg>           | opkg remove <pkg>            | opkg upgrade <pkg>               | opkg find <pkg>            | opkg info <pkg>                 | opkg update              | opkg upgrade             | opkg list --installed             |
| pacman   | pacman -S <pkg>              | pacman -Rs <pkg>             | pacman -S <pkg>                  | pacman -Ss <pkg>           | pacman -Si <pkg>                | pacman -Syy              | pacman -Syu              | pacman -Qe                        |
| pkg      | pkg install <pkg>            | pkg remove <pkg>             | pkg install <pkg>                | pkg search <pkg>           | pkg info <pkg>                  | pkg update               | pkg upgrade              | pkg info -a/--all                 |
| pkg(2)   | pkg install <pkg>            | pkg uninstall <pkg>          | pkg install <pkg>                | pkg search <pkg>           | pkg show <pkg>                  | pkg update               | pkg upgrade              | pkg list-installed                |
| scoop    | scoop install <pkg>          | scoop uninstall <pkg>        | scoop update <pkg>               | scoop search <pkg>         | scoop info <pkg>                | scoop update             | scoop update *           | scoop list                        |
| slackpkg | slackpkg install <slackpkg>  | slackpkg remove <slackpkg>   | slackpkg upgrade <slackpkg>      | slackpkg search <slackpkg> | slackpkg info <slackpkg>        | slackpkg update          | slackpkg upgrade-all     | ls -1 /var/log/packages           |
| snap     | snap install --classic <pkg> | snap remove <pkg>            | snap refresh <pkg>               | snap find <pkg>            | snap info <pkg>                 | -                        | snap refresh             | snap list                         |
| upt      | upt install <pkg>            | upt remove <pkg>             | upt upgrade <pkg>                | upt search <pkg>           | upt info <pkg>                  | upt update               | upt upgrade              | upt list                          |
| urpm     | urpmi <pkg>                  | urpme <pkg>                  | urpmi <pkg>                      | urpmq -y/--fuzzy <pkg>     | urpmq -i <pkg>                  | urpmi.update -a          | urpmi --auto-update      | rpm -q/--query --all              |
| winget   | winget install <pkg>         | winget uninstall <pkg>       | winget upgrade <pkg>             | winget search <pkg>        | winget show <pkg>               | -                        | winget upgrade --all     | winget list                       |
| xbps     | xbps-install <pkg>           | xbps-remove <pkg>            | xbps-install -u/--update <pkg>   | xbps-query -Rs <pkg>       | xbps-query -RS <pkg>            | xbps-install -S/--sync   | xbps-install -u/--update | qxbps-query -l/--list-pkgs        |
| yum      | yum install <pkg>            | yum remove <pkg>             | yum upgrade <pkg>                | yum search <pkg>           | yum info <pkg>                  | yum check-update         | yum update               | yum list --installed              |
| zypper   | zypper install <pkg>         | zypper remove <pkg>          | zypper update <pkg>              | zypper search <pkg>        | zypper info <pkg>               | zypper refresh           | zypper update            | zypper search -i/--installed-only |
```

### Known OSs

```
+------------------------------------------------------+----------------------+
| OS                                                   | Tools                |
+------------------------------------------------------+----------------------+
| windows                                              | scoop, choco, winget |
+------------------------------------------------------+----------------------+
| macos                                                | brew, port           |
+------------------------------------------------------+----------------------+
| ubuntu, debian, linuxmint, pop, deepin, elementray   | apt                  |
| kali, raspbian, aosc, zorin, antix, devuan, bodhi    |                      |
| lxle, sparky                                         |                      |
+------------------------------------------------------+----------------------+
| fedora, redhat, rhel, amzn, ol, almalinux, rocky     | dnf, yum             |
| oubes, centos, qubes, eurolinux                      |                      |
+------------------------------------------------------+----------------------+
| arch, manjaro, endeavouros, arcolinux, garuda        | pacman               |
| antergos, kaos                                       |                      |
+------------------------------------------------------+----------------------+
| alpine, postmarket                                   | apk                  |
+------------------------------------------------------+----------------------+
| opensuse, opensuse-leap, opensuse-tumbleweed         | zypper               |
+------------------------------------------------------+----------------------+
| nixos                                                | nix-env              |
+------------------------------------------------------+----------------------+
| gentoo, funtoo                                       | emerge               |
+------------------------------------------------------+----------------------+
| void                                                 | xbps                 |
+------------------------------------------------------+----------------------+
| mageia                                               | urpm                 |
+------------------------------------------------------+----------------------+
| slackware                                            | slackpkg             |
+------------------------------------------------------+----------------------+
| solus                                                | eopkg                |
+------------------------------------------------------+----------------------+
| openwrt                                              | opkg                 |
+------------------------------------------------------+----------------------+
| nutyx                                                | cards                |
+------------------------------------------------------+----------------------+
| freebsd, ghostbsd                                    | pkg                  |
+------------------------------------------------------+----------------------+
| android                                              | pkg(2)               |
+------------------------------------------------------+----------------------+
```

Some platforms may support multiple package management tools, and upt selects one of them in the order listed in the table.


You can also use the environment variable `UPT_TOOL` to specify the package management tool instead of letting upt automatically determine it.

```sh
UPT_TOOL=nix upt install vim              # use nix-env to install vim
UPT_TOOL=snap upt install vim             # use snap to install vim
```

## Install

**Use Cargo**

Upt is written in the rust you can install it using [cargo](https://doc.rust-lang.org/stable/cargo/).

```sh
cargo install upt
```

**Binaries for macOS, Linux, Windows, BSD**

Download it from [GitHub Releases](https://github.com/sigoden/upt/releases), unzip and add `upt` to your $PATH.

## Usage

```
Usage: 
  upt install <pkg>       Install packages
  upt remove <pkg>        Remove packages
  upt upgrade <pkg>       Upgrade packages
  upt search <pkg>        Search for packages
  upt info <pkg>          Show package details
  upt update              Update package indexes
  upt upgrade             Upgrade all packages
  upt list                List all installed packages

Automatically confirm the action with: -y/--yes
```

## License

Copyright (c) 2023 upt-developers.

Upt is made available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the LICENSE-APACHE and LICENSE-MIT files for license details.
