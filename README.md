# PGR

This is a very simple password generator written in rust.

## Installation

Simply run this one command after cloning the repo:

```bash
cargo install --path .
```

## Usage

```
pgr 
Password Generator Rust is a cli to generate secure passwords locally and blazingly fast

USAGE:
    pgr [OPTIONS]

OPTIONS:
    -d, --digit              Should digits be excluded
    -h, --help               Print help information
    -l, --length <LENGTH>    The length of the password [default: 16]
    -L, --lowercase          Should lowercase letters be excluded
    -s, --special            Should special charachters be excluded
    -u, --uppercase          Should uppercase letteres be excluded

For more information and the source code, visit https://github.com/dlurak/pgr.
```
