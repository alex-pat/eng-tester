# Eng-tester

[![Build Status](https://travis-ci.org/alex-pat/eng-tester.svg?branch=master)](https://travis-ci.org/alex-pat/eng-tester)

Testing system based on org-mode table files. In fact, it's my [old english tester](https://gitlab.com/alex-pat/english)
on steroids, with TUI, rewritten in Rust. Tables are placed in `res/` dir.

## Building

```shell
cargo build --release
```

## Usage

```shell
target/release/eng-tester <file.org> [--gui]
```
