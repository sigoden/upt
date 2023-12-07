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

![Commands](https://github.com/sigoden/upt/assets/4012553/7e629471-6499-439a-9692-d296cd669a9b)


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

## License


Copyright (c) 2023 sigoden

Licensed under the MIT license.
