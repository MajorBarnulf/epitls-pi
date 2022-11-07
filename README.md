# epitls PI

## Description

A little helper tool meant to ease the developpment of the C homeworks at
EPITA/Toulouse.

## Usage

```sh
epitls-pi 

USAGE:
    pi <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    check    Checks a source file for conformance with piscine limitations
    help     Print this message or the help of the given subcommand(s)
    init     Initializes a project directory configuration, useful for custom flags, includes and custop push messages
    run      Runs a set of files or the default target
    test     Runs tests contained within a particular test file or the default test file
    watch    Watches changes to the project included files and runs a command on changes
```

## Installation

- through the AUR, as `epitls-pi-bin`
- with cargo `cargo install epitls-pi`

## TODO

- [ ] add support and switch on run/test for strict mode.
- [x] add `-p` flag to run subcommand with parameter piping.
- [ ] flag on push to add automatically
- [ ] prevent double includes.
- [ ] flag on init to copy personnal environment
- [ ] gc subcommand to free cache
