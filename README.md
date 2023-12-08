# Upt — **U**niversal **P**ackage-management **T**ool.

Upt provides a set of commands to manage packages for all OSs. 

Upt is just an advanced alias, relying on the platform's package management tool to do the job.

[![Build status](https://github.com/sigoden/aichat/actions/workflows/ci.yaml/badge.svg)](https://github.com/sigoden/upt/actions)
[![Crates.io](https://img.shields.io/crates/v/upt.svg)](https://crates.io/crates/upt)

## Features

### Unified Interface

Each operating system (OS) has its own package management tool, which requires different commands to complete the same operation.
This can be inconvenient when switching between or trying new OSs. 

```sh
apt list --upgradable       # Ubuntu, Debian, Linux Mint...
yum list updates          # Centos, Redhat...
brew outdated             # MacOS Homebrew
apk list --upgrades       # Alpine
pacman -Qu                # Arch, Manjaro...
choco outdated            # Windows Chocolatey
dnf list --upgrades       # Fedora
```

However, `upt` offers a solution by providing a unified command for package management operations across various platforms and distributions. 

```sh
upt install vim # Works on any OS
```

### Act as other command

Upt can act as other commands and use their syntax.

```sh
cp upt brew
brew install vim # use brew syntax

cp upt pacman
pacman -S vim    # use pacman syntax
```

### Supported Tools

```
+--------+--------------------+----------------------+--------------------+-----------------------+-----------------+--------------------------+-------------------+---------------------+----------------------+
| Tool   | Install            | Uninstall            | Upgrade            | Search                | Info            | Update Index             | Upgrade All       | List upgradable     | List Installed       |
+--------+--------------------+----------------------+--------------------+-----------------------+-----------------+--------------------------+-------------------+---------------------+----------------------+
| upt    | upt install $pkg   | upt remove $pkg      | upt upgrade $pkg   | upt search $pattern   | upt show $pkg   | upt update               | upt upgrade       | upt list -u         | upt list -i          |
+--------+--------------------+----------------------+--------------------+-----------------------+-----------------+--------------------------+-------------------+---------------------+----------------------+
| apt    | apt install $pkg   | apt remove $pkg      | apt install $pkg   | apt search $pattern   | apt show $pkg   | apt update               | apt upgrade       | apt list --upgrade  | apt list --installed |
+--------+--------------------+----------------------+--------------------+-----------------------+-----------------+--------------------------+-------------------+---------------------+----------------------+
| brew   | brew install $pkg  | brew uninstall $pkg  | brew upgrade $pkg  | brew search $pattern  | brew info $pkg  | brew update              | brew upgrade      | brew outdated       | brew list            |
+--------+--------------------+----------------------+--------------------+-----------------------+-----------------+--------------------------+-------------------+---------------------+----------------------+
| choco  | choco install $pkg | choco uninstall $pkg | choco upgrade $pkg | choco search $pattern | choco info $pkg | choco upgrade all --noop | choco upgrade all | choco outdated      | choco list           |
+--------+--------------------+----------------------+--------------------+-----------------------+-----------------+--------------------------+-------------------+---------------------+----------------------+
| dnf    | dnf install $pkg   | dnf remove $pkg      | dnf update $pkg    | dnf search $pattern   | dnf info $pkg   | dnf check-update         | dnf update        | dnf list --upgrades | dnf list --installed |
+--------+--------------------+----------------------+--------------------+-----------------------+-----------------+--------------------------+-------------------+---------------------+----------------------+
| yum    | yum install $pkg   | yum remove $pkg      | yum upgrade $pkg   | yum search $pattern   | yum info $pkg   | yum check-update         | yum update        | yum list updates    | yum list installed   |
+--------+--------------------+----------------------+--------------------+-----------------------+-----------------+--------------------------+-------------------+---------------------+----------------------+
| pacman | pacman -S $pkg     | Pacman -Rs $pkg      | pacman -S $pkg     | pacman -Ss $pattern   | pacman -Si $pkg | pacman -Syy              | pacman -Syu       | pacman -Qu          | pacman -Qe           |
+--------+--------------------+----------------------+--------------------+-----------------------+-----------------+--------------------------+-------------------+---------------------+----------------------+
| apk    | apk add $pkg       | apk del $pkg         | apk upgrade $pkg   | apk search $pattern   | apk info $pkg   | apk update               | apk upgrade       | apk list --upgrades | apk list --installed |
+--------+--------------------+----------------------+--------------------+-----------------------+-----------------+--------------------------+-------------------+---------------------+----------------------+
```

### Supported OSs

```
+-------------------------------------------+----------------------+
| OS                                        | Tools                |
+-------------------------------------------+----------------------+
| windows                                   | scoop, choco, winget |
+-------------------------------------------+----------------------+
| macos                                     | brew, port           |
+-------------------------------------------+----------------------+
| ubuntu, debian, linuxmint, pop, deepin | apt                  |
| elementary OS, kali, aosc                  |                      |
+-------------------------------------------+----------------------+
| fedora, redhat, rhel                      | dnf                  |
+-------------------------------------------+----------------------+
| centos, rocky                             | yum                  |
+-------------------------------------------+----------------------+
| alpine                                    | apk                  |
+-------------------------------------------+----------------------+
```

Some platforms may support multiple package management tools, and upt defaults to selecting them in the order listed in the table. 
Of course, you can also specify the `UPT_TOOL` environment variable.

For example, in Windows, `upt` will prioritize using `scoop`. If `scoop` is not available, it will use `choco`. If `choco` is also not available, it will use `winget`. If you set `UPT_TOOL=choco`, `choco` will be used even though `scoop` exists.

## Install

**Download Binary**

Binary are available for download [Releases](https://github.com/sigoden/upt/releases) . Make sure to put the path to the binary into your `PATH`.

```sh
mv upt /usr/local/bin # Linux, MacOS
```

```bat
:: Windows, Run as administrator
move upt C:\Windows\System32
```

**Use Cargo**

`upt` is written in the rust you can install it using [cargo](https://doc.rust-lang.org/stable/cargo/).

```sh
cargo install upt
```

## Usage

- Install packages

```sh
upt install vim                 # install a single package
upt install vim ripgrep         # install multiple packages
upt install -y vim              # assume yes when installing
```

- Remove packages

```sh
upt remove vim                  # remove a single package
upt remove vim ripgrep          # remove multiple packages
upt remove -y vim               # assume yes when removing
```

- Upgrade packages

```sh
upt upgrade vim                 # upgrade a single package
upt upgrade vim ripgrep         # upgrade multiple packages
upt upgrade -y vim              # assume yes when upgrading
```

- Search for package

```sh
upt search vim
```

- Show package details

```sh
upt show vim
```

- Update indexes of packages

```sh
upt update
```

- Upgrade all packages

```sh
upt upgrade
upt upgrade -y                  # assume yes when upgrading
```

- List all upgradable packages

```sh
upt list -i
```

- List all installed packages

```sh
upt list -u
```
> If you feel `upt` should support a package management operation, feel free to create [issue](https://github.com/sigoden/upt/issues/new) to discuss it.

## License

Copyright (c) 2023 argc-developers.

aichat is made available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the LICENSE-APACHE and LICENSE-MIT files for license details.
