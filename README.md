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

## Scalar Types

A scalar type represents a single value. Rust has four primary scalar types:

- integers
- floating-point numbers
- Booleans
- characters

### Integer Types

| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

The isize and usize types depend on the architecture of the computer your program is running on, which is denoted in the
table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

#### Integer Literals in Rust

| Number literals | Example     |
|-----------------|-------------|
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        | 


### Floating-Point Types

- `f32`: 32 bits in size (single-precision float)
- `f64`: 64 bits in size (double precision float)

The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All
floating-point types are signed. All floating-point types are signed:

## Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types:

- tuples
- arrays

### The Tuple Type

A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples
have a fixed length: once declared, they cannot grow or shrink in size.

We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type,
and the types of the different values in the tuple don’t have to be the same. We’ve added optional type annotations in
this example:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable tup binds to the entire tuple because a tuple is considered a single compound element. To get the
individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

The tuple without any values has a special name, `unit`. This value and its corresponding type are both written `()` and
represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any
other value.

### The Array Type

Unlike a tuple, every element of an array must have the `same type`. Unlike arrays in some other languages, arrays in
Rust have a `fixed length`.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of
elements in the array, like so:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

You can also initialize an array to contain the same value for each element by specifying the initial value, followed by
a semicolon, and then the length of the array in square brackets, as shown here:

```rust
let a = [3; 5]; // same as let a = [3, 3, 3, 3, 3]
```

## Functions

Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase
and underscores separate words.

Rust does not care where you define your functions, only that they’re defined somewhere in a scope that can be seen by
the caller.

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```
