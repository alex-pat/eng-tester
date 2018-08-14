# Eng-tester

[![Linux Build Status](https://travis-ci.org/alex-pat/eng-tester.svg?branch=master)](https://travis-ci.org/alex-pat/eng-tester)
[![Windows build status](https://ci.appveyor.com/api/projects/status/github/alex-pat/eng-tester?svg=true)](https://ci.appveyor.com/project/alex-pat/eng-tester)

Testing system based on org-mode table files. In fact, it's my [old english tester](https://gitlab.com/alex-pat/english)
on steroids, with TUI, rewritten in Rust. Tables are placed in `res/` dir.

## Building

```shell
$ cargo build --release
```

You can build it without curses feature by adding `--no-default-features`.

## Usage

```shell
$ cargo run -- --help
eng_tester 0.1.0
Alexander Pateenok <pateenoc@gmail.com>
Testing system based on org-mode table files.

USAGE:
    eng_tester [FLAGS] <file>

FLAGS:
    -h, --help       Prints help information
    -g, --gui        gui-mode
    -V, --version    Prints version information

ARGS:
    <file>    org-mode file with the table
```
