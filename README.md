# Upt
**U**niversal **P**ackage-management **T**ool for Windows, macOS and Linux.

Upt recongizes your input, and interprets it into the command which invokes the native package management tool.

For example, if your input `brew install vim` or `upt install vim` on `Ubutu`, upt will recongize that your want to install vim, then execute `apt install vim`.

## Install

- **From binaries**
   Binaries are available for download [here][releases]. Make sure to put the path to the binary into your `PATH`.

<!-- - **From Crates.io**

   This requires at least [Rust] 1.20 and Cargo to be installed. Once you have installed
   Rust, type the following in the terminal:

   ```
   cargo install upt
   ```

   This will download and compile upt for you, the only thing left to do is
   to add the Cargo bin directory to your `PATH`. -->

- **For Contributions**  

   If you want to contribute to upt  you will have to clone the repository on
   your local machine:

   ```
   git clone https://github.com/sigoden/upt.git
   ```

   `cd` into `upt/` and run

   ```
   cargo build
   ```

   The resulting binary can be found in `upt/target/debug/` under the name
   `upt` or `upt.exe`.

## Usage

Upt is not only a command line tool but also an interpreter.

As a command line, it can:

- Install packges: `upt install cargo`
- Remove packages: `upt remove cargo`
- Remove packages: `upt upgrade cargo`
- Search for packges:  `upt search cargo`
- Show package details:  `upt show cargo`
- Show indexes of packages:  `upt update`
- Upgrade all packages :  `upt upgrade`
- List installed packages: `upt -i`
- List upgradable packages: `upt -u`

> For general purpose, upt only provide most general and useful commands.

As a interpreter, it can act as other command line tool.

Rename `upt` to `pacman`, your can take advantage of your `pacman` experience to manage packages. It becomes `pacman`. Type `pacman -S cargo` to install cargo other than `pacman install cargo`. 

```
| task                       | udt              | apt                   | brew                | choco                | yum                | dnf                  | pacman          | apk                  |
| :------------------------- | :--------------- | :-------------------- | :------------------ | :------------------- | :----------------- | :------------------- | :-------------- | :------------------- |
| Install packages           | udt install $pkg | apt install $pkg      | brew install $pkg   | choco install $pkg   | yum install $pkg   | dnf install $pkg     | pacman -S $pkg  | apk add $pkg         |
| Remove packages            | udt remove $pkg  | apt remove $pkg       | brew uninstall $pkg | choco uninstall $pkg | yum remove $pkg    | dnf remove $pkg      | pacman -Rs $pkg | apk del $pkg         |
| Upgrade packages           | udt upgrade $pkg | apt install $pkg      | brew upgrade $pkg   | choco upgrade $pkg   | yum update $pkg    | dnf upgrade $pkg     | pacman -S $pkg  | apk upgrade $pkg     |
| Search for package         | udt search $pkg  | apt search $pkg       | brew search $pkg    | choco search $pkg    | yum search $pkg    | dnf search $pkg      | pacman -Ss $pkg | apk search $pkg      |
| Show package details       | udt show $pkg    | apt show $pkg         | brew info $pkg      | choco info $pkg      | yum info $pkg      | dnf info $pkg        | pacman -Si $pkg | apk info $pkg        |
| Update indexes of packages | udt update       | apt update            | brew update         | choco upgrade --noop | yum check-update   | dnf check-update     | pacman -Syy     | apk update           |
| Upgrade all packages       | udt upgrade      | apt upgrade           | brew upgrade        | choco upgrade all    | yum update         | dnf upgrade          | pacman -Syu     | apk upgrade          |
| List upgradable packages   | udt list -u      | apt list --upgradable | brew outdated       | choco outdated       | yum list updates   | dnf list --upgrades  | pacman -Qu      | apk list --upgrades  |
| List installed packages    | udt list -i      | apt list --installed  | brew list           | choco list -lai      | yum list installed | dnf list --installed | pacman -Qe      | apk list --installed |
```

## License

Copyright (c) 2019 sigoden

Licensed under the MIT license.