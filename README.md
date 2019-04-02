# Upt — **U**niversal **P**ackage-management **T**ool.

[![Linux build status](https://travis-ci.org/sigoden/upt.svg)](https://travis-ci.org/sigoden/upt)
[![Windows build status](https://ci.appveyor.com/api/projects/status/github/sigoden/upt?svg=true)](https://ci.appveyor.com/project/sigoden/upt)
[![Crates.io](https://img.shields.io/crates/v/upt.svg)](https://crates.io/crates/upt)

English | [简体中文](./README-zh_CN.md)

## Features

**Unified Command**

Each OS has its own package management tool. To complete the same operation, you need to enter different commands.

For example, we need to get an updateable package, We have to execute the following command:

```sh
apt list --upgrable       # Ubuntu, Debian, Linux Mint...
yum list updates          # Centos, Redhat...
brew outdated             # MacOS Homebrew
apk list --upgrades       # Apline
pacman -Qu                # Arch, Manjaro...
choco outdated            # Windows Chocolaty
dnf list --upgrades       # Fedora
```

This brings us a lot of inconvenience in switching and experiencing the new OS. After all, package management is a essentail and common operation.

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
upt remove -y vim               # assume yes when removeing
```

- Upgrade packages

```sh
upt upgrade vim                 # upgrade a single package
upt upgrade vim ripgrep         # upgrade multiple packages
upt upgrade -y vim              # assume yes when upgradeing
```

- Search for pacakge

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
upt upgrade -y                  # assume yes when upgradeing
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
 - After parsing, `upt` find that you are going to execute task: `list all updateable packages'.
 - `upt` detects the `apt` package management tool used by your system.
 - `upt` interpret the task `list all updateable packages` to `apt` command.
 - Run `apt list --upgradable`.

If you rename `upt` to `brew`, `upt` will use the `brew` syntax to resolve the task. **Command Replacement** is implemented this way.

The following table is the input and output mapping:

```
| task                         | udt              | apt                   | brew                | choco                | yum                | dnf                  | pacman          | apk                  |
| :--------------------------- | :--------------- | :-------------------- | :------------------ | :------------------- | :----------------- | :------------------- | :-------------- | :------------------- |
| Install packages             | udt install $pkg | apt install $pkg      | brew install $pkg   | choco install $pkg   | yum install $pkg   | dnf install $pkg     | pacman -S $pkg  | apk add $pkg         |
| Remove packages              | udt remove $pkg  | apt remove $pkg       | brew uninstall $pkg | choco uninstall $pkg | yum remove $pkg    | dnf remove $pkg      | pacman -Rs $pkg | apk del $pkg         |
| Upgrade packages             | udt upgrade $pkg | apt install $pkg      | brew upgrade $pkg   | choco upgrade $pkg   | yum update $pkg    | dnf upgrade $pkg     | pacman -S $pkg  | apk upgrade $pkg     |
| Search for package           | udt search $pkg  | apt search $pkg       | brew search $pkg    | choco search $pkg    | yum search $pkg    | dnf search $pkg      | pacman -Ss $pkg | apk search $pkg      |
| Show package details         | udt show $pkg    | apt show $pkg         | brew info $pkg      | choco info $pkg      | yum info $pkg      | dnf info $pkg        | pacman -Si $pkg | apk info $pkg        |
| Update indexes of packages   | udt update       | apt update            | brew update         | choco upgrade --noop | yum check-update   | dnf check-update     | pacman -Syy     | apk update           |
| Upgrade all packages         | udt upgrade      | apt upgrade           | brew upgrade        | choco upgrade all    | yum update         | dnf upgrade          | pacman -Syu     | apk upgrade          |
| List all upgradable packages | udt list -u      | apt list --upgradable | brew outdated       | choco outdated       | yum list updates   | dnf list --upgrades  | pacman -Qu      | apk list --upgrades  |
| List all installed packages  | udt list -i      | apt list --installed  | brew list           | choco list -lai      | yum list installed | dnf list --installed | pacman -Qe      | apk list --installed |
```

The table also lists:

  - commands that can be replaced
  - Supported package management tools
  - Syntax for inter-command conversion

If you find errors or want to add other package management tools, please [issue](https://github.com/sigoden/upt/issues/new).

## License


Copyright (c) 2019 sigoden

Licensed under the MIT license.