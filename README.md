# rust101

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/1995parham/rust101/ci?label=ci&logo=github&style=flat-square)

## Introduction

Easy to understand applications with rust just for having fun :relieved:

## BPF

To write BPF code in Rust, it's easiest to use cargo-bpf (part of the redbpf suite)
which handles setting up the project and can even function as a development loader.

```sh
sudo pacman -Syu llvm
cargo install cargo-bpf --features=llvm13
```

```sh
cargo new elbpf
cargo bpf add hello_world
```

## Projects

### Phone Book :iphone:

Phone book application stores users and their phones as struct. It has an uncomplicated menu, and users can add, get, and remove entities.

### Word Count 🔢

This application receives a delimiter and some entries (i.e. some text files or array of strings) and after separation based on a delimiter, it will count and calculate the number of each word in the entries.
