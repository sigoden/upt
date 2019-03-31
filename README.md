# Universal Package Tool

## Why upt

- Provide general interface for package management
- Manage package in your most favoriate and most familar way

## Translation Map

| purpose                 | udt                   | apt                    | brew                    | choco                    | yum                  | dnf                  | pkg                  | pacman              |
| ----------------------- | --------------------- | ---------------------- | ----------------------- | ------------------------ | -------------------- | -------------------- | -------------------- | ------------------- |
| Install package         | udt install $package  | apt install $package   | brew install $package   | choco install $package   | yum install $package | dnf install $package | pkg install $package | pacman -S $package  |
| Remove package          | udt remove $package   | apt uninstall $package | brew uninstall $package | choco uninstall $package | yum remove $package  | dnf remove $package  | pkg delete $package  | pacman -Rs $package |
| Upgrade package         | udt upgrade $package  | apt install $package   | brew upgrade $package   | choco upgrade $package   | yum update $package  | dnf upgrade $package | pkg upgrade $package | pacman -S $package  |
| Search for package      | udt search $package   | apt search $package    | brew search $package    | choco search $package    | yum search $package  | dnf search $package  | pkg search $package  | pacman -Ss $package |
| Show package info       | udt show $package     | apt show $package      | brew info $package      | choco info $package      | yum info $package    | dnf info $package    | pkg info $package    | pacman -Si $package |
| Update package datebase | udt update            | apt update             | brew update             | choco upgrade --noop     | yum check-update     | dnf check-update     | pkg check-update     | pacman -Syy         |
| Upgrade all packages    | udt upgrade           | apt upgrade            | brew upgrade            | choco upgrade all        | yum update           | dnf upgrade          | pkg upgrade          | pacman -Syu         |
| List installed package  | udt list --installed  | apt list --installed   | brew list               | choco list -lai          | yum list installed   | dnf list --installed | pkg info             | pacman -Qe          |
| List upgradable package | udt list --upgradable | apt list --upgradable  | brew outdated           | choco outdated           | yum list updates     | dnf list --upgrades  | pkg list updates     | pacman -Qu          |

## License

Copyright (c) 2019 sigoden

Licensed under the MIT license.