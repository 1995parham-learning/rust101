<div align="center">
<h1>Rust 101</h1>
<h6>Learning Rust with the love of your life</h6>
<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/1995parham-learning/rust101/ci.yaml?style=for-the-badge">
</div>

## Introduction

Easy to understand applications with Rust just for having fun :relieved:.

## Tools

### rustc

The Rust compiler

### cargo

The Rust dependency/project manager

### Rustup

Rustup manages the rust versions, required programs, etc. It is a great program to simplify your rust programming workflow.

## Dependencies

Rust handles dependencies by hand in `cargo.toml` file. In this file you specify the dependencies by their required version.

```bash
# Create a new Cargo package
carg new

# Build and install a Rust binary
cargo install

# Compile the current package
cargo build

# Search packages in crates.io
cargo search

# Build a package's documentation
cargo doc

# Execute unit and integration tests of a package
cargo test

# Automatically fix lint warnings reported by rustc
cargo fix

# Execute benchmarks of a package
cargo bench
```

For finding about latest release of each cargo you can use [Docs.rs](https://docs.rs).

## Expression vs Statement

Statement is instruction that performs some action and do not return a value.
Expressions evaluate to a resulting value.

The `let y = 6` statement doesn't return a value. This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment.

These are expressions:

- The `6` in the statement `let y = 6;`
- Calling a function
- Calling a macro
- {}

## Print

rust has `println!` macro which has `{}` for normal printing, `{:?}` for debug printing and `{:p}` for pointers.

## Match

```rust
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}

let some\*u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
      2 => println!("two"),

      - => (),
}

# [derive(Debug)]

enum UsState {
    Alabama,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

match coin {
    Coin::Penny => 1,
    Coin::Nickel => 2,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
        println!("State quarter from {:?}!", state);
        25
    },
}

```

## Loops

```rust
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

```rust
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element)
}

for number in (1..4).rev() {
    println!("{}!", number)
}
```

## Stack vs. Heap

All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size might change must be stored on the heap instead.
string literals (`str` type) are stored on executable and we know them at the compile time. `String`s are stored on heap and we get their memory at the runtime from the heap.
We need to free heap memory and rust will do this by calling the `drop` function on that complex type.

## Ownership

Assignments in rust may copy or move the variable if variable has the copy trait rust will uses copy otherwise it uses move.
types that doesn't have drop method in them or their parts can have copy trait otherwise there would be a compile error unless you implement the clone trait.
With references you can prevent moving variables.

## Iterators

iterators have method for casting them into collections which is named /collect/.

## Arrays

Rust arrays have fixed size and can be defined in the following ways:

```rust
let a1 = [1, 2, 3]
let a2: [u64; 5] = [1, 2, 3, 4, 5]

```

Invalid array access doesn't cause compile error in Rust but it cause panic at runtime. Go in this case creates compile error.

## Slice

Slice is reference to its underlying array:

```rust

let s = String::from("hello")

let slice = &s[..]
let slice = &s[0..]

```

## Traits

traits are similar to interface in go. they can be implemented on structures or their references.

## Github Actions

[Unofficial GitHub Actions for Rust programming language](https://github.com/actions-rs)

## To Read

- [writing an os in rust](https://github.com/phil-opp/blog_os)

## Crates

- [clap is a simple-to-use, efficient, and full-featured library for parsing command line arguments and subcommands when writing console/terminal applications.](https://docs.rs/clap/)
- [Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.](https://serde.rs)
- [The reqwest crate provides a convenient, higher-level HTTP Client.](https://docs.rs/reqwest)
- [Rust client for NATS, the cloud native messaging system.](https://github.com/nats-io/nats.rs)
- [Tokio is an asynchronous runtime for the Rust programming language.](https://tokio.rs)
- [Termion is a pure Rust, bindless library for low-level handling, manipulating and reading information about terminals. This provides a full-featured alternative to Termbox.](https://docs.rs/termion)

## Projects

### Phone Book 📱

Phone book application stores users and their phones as struct.
It has an uncomplicated menu,
and users can add, get, and remove entities.

### Word Count 🔢

This application receives a delimiter and some entries
(i.e. some text files or array of strings) and after separation based on a delimiter,
it will count and calculate the number of each word in the entries.

### Data in Depth

This example is based on _Rust in Action_ book and shows how data is stored.
One of its example use rust overflow panic and
if you build it in release mode then you don't have it.

### BPF

To write BPF code in Rust, it's easiest to use cargo-bpf (part of the `redbpf` suite)
which handles setting up the project and can even function as a development loader.

```bash
sudo pacman -Syu llvm
cargo install cargo-bpf --features=llvm13
```

```bash
cargo bpf new elbpf
cargo bpf add hello
```
