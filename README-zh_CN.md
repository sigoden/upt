# UPT —— 统一的包管理工具

[![Linux build status](https://travis-ci.org/sigoden/upt.svg)](https://travis-ci.org/sigoden/upt)
[![Windows build status](https://ci.appveyor.com/api/projects/status/github/sigoden/upt?svg=true)](https://ci.appveyor.com/project/sigoden/upt)
[![Crates.io](https://img.shields.io/crates/v/upt.svg)](https://crates.io/crates/upt)

[English](./README.md) | 简体中文

## 特性

**统一命令**

每款操作系统都有自己的包管理。完成同一各操作，在各个系统中需要输入不同的命令和参数。

比如我们要获取可更新的软件包，必须执行如下命令：

```sh
apt list --upgrable       # Ubuntu, Debian, Linux Mint...
yum list updates          # Centos, Redhat...
brew outdated             # MacOS Homebrew
apk list --upgrades       # Apline
pacman -Qu                # Arch, Manjaro...
choco outdated            # Windows Chocolaty
dnf list --upgrades       # Fedora
```

这给我们切换和体验新系统带来了很大的不便，毕竟包管理是基本而有常用的操作。

`upt` 解决了这个问题。它可以运行于各个平台，甚至各个发行版中，提供**统一命令**执行包管理操作。

```sh
upt list -u # 所有平台命令一致。
```

> UPT 只是提供统一的界面，包管理功能是通过调用系统自带工具实现。


**命令替换**

每个人都有自己熟悉的操作系统和包管理工具。

常用 MacOS 的人可能对 `brew` 命令较熟悉，常用 Windows 的人对 `choco` 较熟悉，常用 Ubuntu 的人对 `apt` 较熟悉。

为了使用 `upt`，还得学套 `upt` 的命令，这并不轻松。幸好，`upt` 支持命令替换。你不再需要学一个新命令。

如果你是一个熟悉 `brew` 的人，在 Ubuntu 中进行包管理时，可以下载 `upt` 命令后，**重命名 upt** 为 `brew`。你可以在 Ubuntu 中使用 `brew` 进行包管理了。

## 安装

**下载可执行文件**

从 [Releases](https://github.com/sigoden/upt/releases) 下载可执行文件。解压后是个单一的可执行文件，将文件复制的合适的路径，并确保路径已加入到 `PATH` 环境变量中。

```sh
mv upt /usr/local/bin # Linux, MacOS
```

```bat 
:: Windows, 以管理员身份运行
move upt C:\Windows\System32 
```

**使用 Cargo 安装**

`upt` 使用 rust 语言编写，并发布到 [crates](https://crates.io/crates/upt)。所以可以使用 [cargo](https://doc.rust-lang.org/stable/cargo/) 安装。

```sh
# curl https://sh.rustup.rs -sSf | sh # 安装 cargo
cargo install upt
```

## 使用

- 安装软件包

```sh
upt install vim                 # 安装单个包
upt install vim ripgrep         # 安装多个
upt install -y vim              # 安装时自动完成 yes 输入
```

- 删除软件包 

```sh
upt remove vim                  # 删除单个包
upt remove vim ripgrep          # 删除多个
upt remove -y vim               # 删除时自动完成 yes 输入
```

- 更新软件包

```sh
upt upgrade vim                 # 更新单个包
upt upgrade vim ripgrep         # 更新多个
upt upgrade -y vim              # 更新时自动完成 yes 输入
```

- 查找软件包

```sh
upt search vim
```

- 查看软件包详情

```sh
upt show vim
```

- 更新软件索引

```sh
upt update
```

- 更新系统

```sh
upt upgrade                    # 更新所有过期的软件包
upt upgrade -y                 # 更新时自动完成 yes 输入
```

- 查看所有已安装的软件包

```sh
upt list -i
```

- 查看所有可更新的软件包

```sh
upt list -u
```

> 如果觉得应该支持某个包管理操作，欢迎发 [Issue](https://github.com/sigoden/upt/issues/new) 一起讨论。

## 指令替换表

`upt` 可执行文件很小，又需要全平台通用，不可能*自带*包管理功能。它通过调用系统自带工具实现包管理功能。所以 `upt` 实质上是一个解释器。

如果你在 Ubuntu 平台中使用 `upt`:

 - 输入 `upt list -u`
 - 解析后发现你是要执行 `获取所有可更新软件包` 这一任务
 - 检测到你的系统使用的 `apt` 包管理工具
 - 将任务 `获取所有可更新软件包` 转换成 `apt` 命令
 - 执行命令 `apt list --upgradable`

如果你将 `upt` 重命名成 `brew`，`upt` 会使用 `brew` 的语法解析成任务。**命令替换**就是如此实现的。

下表是输入与输出映射，也就是指令替换：

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

指令替换表同时列出了:

 - 能够替换的命令
 - 支持的包管理工具
 - 命令间转换的语法

> 如果你发现有错误，或者想添加其他包管理工具，欢迎发 [Issue](https://github.com/sigoden/upt/issues/new)。

## License

Copyright (c) 2019 sigoden

Licensed under the MIT license.

[releases]: https://github.com/sigoden/upt/releases