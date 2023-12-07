# Upt — **U**niversal **P**ackage-management **T**ool.

[![Build status](https://github.com/sigoden/aichat/actions/workflows/ci.yaml/badge.svg)](https://github.com/sigoden/upt/actions)
[![Crates.io](https://img.shields.io/crates/v/upt.svg)](https://crates.io/crates/upt)

## Features

**Unified Command**

Each OS has its own package management tool. To complete the same operation, you need to enter different commands.

For example, we need to get an updatable package, We have to execute the following command:

```sh
apt list --upgradable       # Ubuntu, Debian, Linux Mint...
yum list updates          # Centos, Redhat...
brew outdated             # MacOS Homebrew
apk list --upgrades       # Alpine
pacman -Qu                # Arch, Manjaro...
choco outdated            # Windows Chocolatey
dnf list --upgrades       # Fedora
```

This brings us a lot of inconvenience in switching and experiencing the new OS. After all, package management is a essential and common operation.

`upt` can solve this problem. It can run on various platforms and even in various distributions, provided **Unified Command** to execute the package management operations.

```sh
upt list -u # All in one command
```

> UPT only provides a unified interface, and the package management function is implemented by calling the system's native tools.


**Command Replacement**

Everyone has their familiar operating system and package management tool.

People who use MacOS may be familiar with the `brew` command, People who use Windows may be familiar with the `choco` command, People who use Ubuntu may be familiar with the `apt` command。

In order to use `upt`, you have to learn `upt` command, which is not easy. Fortunately, `upt` supports command replacement. You do not need to learn a new command.

If you are a person familiar with `brew`, when you have to manage the package in Ubuntu, you can download the `upt` command. By **rename upt** to `brew`. You can use `brew`-style for package management in Ubuntu.

```sh
cp upt brew
brew install wget # upt will work like brew
```

**Many supported package management tools**

- [x] apk
- [x] apt
- [x] brew
- [x] choco
- [x] scoop
- [x] dnf
- [x] pacman
- [x] yum

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

`upt` is written in the rust language and posted to [crates](https://crates.io/crates/upt). So you can install it using [cargo](https://doc.rust-lang.org/stable/cargo/).

```sh
# curl https://sh.rustup.rs -sSf | sh # 安装 cargo
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

## Command Replacement Table

The `upt` executable file is small, but it needs to be universal across the platform. It is impossible to *embed* package management function.

`upt` implements package management functions by calling the system's native tools. So `upt` is essentially an **interpreter**.

When you run `upt` in Ubuntu:

 - Input `upt list -u`
 - After parsing, `upt` find that you are going to execute task: `list all updatable packages'.
 - `upt` detects the `apt` package management tool used by your system.
 - `upt` interpret the task `list all updatable packages` to `apt` command.
 - Run `apt list --upgradable`.

If you rename `upt` to `brew`, `upt` will use the `brew` syntax to resolve the task. **Command Replacement** is implemented this way.

The following table is the input and output mapping:

| Tool   | Install            | Uninstall            | Upgrade            | Search                | Info            | Update Index             | Upgrade All       | List upgradable     | List Installed       |
| ------ | ------------------ | -------------------- | ------------------ | --------------------- | --------------- | ------------------------ | ----------------- | ------------------- | -------------------- |
| upt    | upt install $pkg   | upt remove $pkg      | upt upgrade $pkg   | upt search $pattern   | upt show $pkg   | upt update               | upt upgrade       | upt list -u         | upt list -i          |
| apt    | apt install $pkg   | apt remove $pkg      | apt install $pkg   | apt search $pattern   | apt show $pkg   | apt update               | apt upgrade       | apt list --upgrade  | apt list --installed |
| brew   | brew install $pkg  | brew uninstall $pkg  | brew upgrade $pkg  | brew search $pattern  | brew info $pkg  | brew update              | brew upgrade      | brew outdated       | brew list            |
| choco  | choco install $pkg | choco uninstall $pkg | choco upgrade $pkg | choco search $pattern | choco info $pkg | choco upgrade all --noop | choco upgrade all | choco outdated      | choco list           |
| dnf    | dnf install $pkg   | dnf remove $pkg      | dnf update $pkg    | dnf search $pattern   | dnf info $pkg   | dnf check-update         | dnf update        | dnf list --upgrades | dnf list --installed |
| yum    | yum install $pkg   | yum remove $pkg      | yum upgrade $pkg   | yum search $pattern   | yum info $pkg   | yum check-update         | yum update        | yum list updates    | yum list installed   |
| pacman | pacman -S $pkg     | Pacman -Rs $pkg      | pacman -S $pkg     | pacman -Ss $pattern   | pacman -Si $pkg | pacman -Syy              | pacman -Syu       | pacman -Qu          | pacman -Qe           |
| apk    | apk add $pkg       | apk del $pkg         | apk upgrade $pkg   | apk search $pattern   | apk info $pkg   | apk update               | apk upgrade       | apk list --upgrades | apk list --installed |

The table also lists:

  - commands that can be replaced
  - Supported package management tools
  - Syntax for inter-command conversion

If you find errors or want to add other package management tools, please [issue](https://github.com/sigoden/upt/issues/new).

## License


Copyright (c) 2019 sigoden

Licensed under the MIT license.
