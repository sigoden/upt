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

- Install packges: `upt install cargo`, `upt install -y cargo code`(No yes/No prompt)
- Remove packages: `upt remove cargo`, `upt remove -y cargo code`(No yes/No prompt)
- Remove packages: `upt upgrade cargo`, `upt upgrade -y cargo code`(No yes/No prompt)
- Search for packges:  `upt search cargo`
- Show package details:  `upt show cargo`
- Show indexes of packages:  `upt update`
- Upgrade all packages :  `upt upgrade`
- List installed packages: `upt -i`
- List upgradable packages: `upt -u`

> For general purpose, upt only provide most general and useful commands.

As a interpreter, it can act as other command line tool.

Rename `upt` to `pacman`, your can take advantage of your `pacman` experience to manage packages. It becomes `pacman`. Type `pacman -S cargo` to install cargo other than `pacman install cargo`。 

| task                       | udt                   | apt                   | brew                    | choco                    | yum                  | dnf                  | pkg                  | pacman              |
| -------------------------- | --------------------- | --------------------- | ----------------------- | ------------------------ | -------------------- | -------------------- | -------------------- | ------------------- |
| Install packages           | udt install $package  | apt install $package  | brew install $package   | choco install $package   | yum install $package | dnf install $package | pkg install $package | pacman -S $package  |
| Remove packages            | udt remove $package   | apt remove $package   | brew uninstall $package | choco uninstall $package | yum remove $package  | dnf remove $package  | pkg delete $package  | pacman -Rs $package |
| Upgrade packages           | udt upgrade $package  | apt install $package  | brew upgrade $package   | choco upgrade $package   | yum update $package  | dnf upgrade $package | pkg upgrade $package | pacman -S $package  |
| Search for package         | udt search $package   | apt search $package   | brew search $package    | choco search $package    | yum search $package  | dnf search $package  | pkg search $package  | pacman -Ss $package |
| Show package details       | udt show $package     | apt show $package     | brew info $package      | choco info $package      | yum info $package    | dnf info $package    | pkg info $package    | pacman -Si $package |
| Update indexes of packages | udt update            | apt update            | brew update             | choco upgrade --noop     | yum check-update     | dnf check-update     | pkg check-update     | pacman -Syy         |
| Upgrade all packages       | udt upgrade           | apt upgrade           | brew upgrade            | choco upgrade all        | yum update           | dnf upgrade          | pkg upgrade          | pacman -Syu         |
| List installed packages    | udt list --installed  | apt list --installed  | brew list               | choco list -lai          | yum list installed   | dnf list --installed | pkg info             | pacman -Qe          |
| List upgradable packages   | udt list --upgradable | apt list --upgradable | brew outdated           | choco outdated           | yum list updates     | dnf list --upgrades  | pkg list updates     | pacman -Qu          |

## License

Copyright (c) 2019 sigoden

Licensed under the MIT license.