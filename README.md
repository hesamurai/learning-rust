# Learning Rust

## Different commands

```bash
# to check Rust version
rustc --version

# to update Rust
rustup update

# to uninstall Rust and rustup
rustup self uninstall

# to load local documentation
rustup doc

# to compile a Rust file
rustc {path to file}

# to check Cargo verion
cargo --version

# to create a new project using Cargo
cargo new {project_root_folder_name}

# to build a Cargo project
cargo build

# to build and run a Cargo project
cargo run

# to check code to make sure it compiles
cargo check

# to update the dependencies
cargo update

# to build documentation provided by all your dependencies locally and open it in your browser
cargo doc --open 
```

## Standard Library

### std::io

`std::io` is used to obtain user input: https://doc.rust-lang.org/std/io/index.html

`std::cmp` is used for comparison purposes: https://doc.rust-lang.org/std/cmp/index.html

## Variables

In Rust, variables are immutable by default:

```bash
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

## Types

- `String` is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text
- `i32` is a 32-bit integer
- `u32` is an unsigned 32-bit integer
