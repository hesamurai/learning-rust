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

## `if` expressions

The condition of an `if` expression must be a `bool`. If the condition isn’t a bool, we’ll get an error. Rust only
executes the block for the first `true` condition, and once it finds one, it does not even check the rest.

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3"); // this line would be hit
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

Because `if` is an expression, we can use it on the right side of a `let` statement to assign the outcome to a variable:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // the following line would print: The value of number is: 5
    println!("The value of number is: {number}");
}
```

## Repetition with Loops

Rust has 3 kinds of loops: `loop`, `while`, and `for`.

The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to
stop:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; // we use a semicolon to end the statement that assigns the value to result

    println!("The result is {result}");
}
```

If you have loops within loops, `break` and `continue` apply to the innermost loop at that point. You can optionally
specify a loop label on a loop that you can then use with `break` or `continue` to specify that those keywords apply to
the labeled loop instead of the innermost loop. Loop labels must begin with a `single quote`. Here’s an example with two
nested loops:

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

which will print:

```bash
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

A `while` loop example:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

A `for` loop example (countdown):

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

## Ownership

These are the rules for ownership:

- Each value in Rust has an `owner`.
- There can only be `one owner at a time`.
- When the owner goes out of scope, `the value will be dropped`.

Rust won’t let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait. If the
type needs something special to happen when the value goes out of scope and we add the `Copy` annotation to that type,
we’ll get a compile-time error.

Here are some of the types that implement `Copy`:

- All the integer types, such as `u32`.
- The Boolean type, `bool`, with values `true` and `false`.
- All the floating-point types, such as `f64`.
- The character type, `char`.
- Tuples, if they only contain types that also implement Copy. For example, `(i32, i32)` implements Copy, but `(i32, 
String)` does not.

```rust
fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s);            // s's value moves into the function and so is no longer valid here

    let x = 5;                     // x comes into scope

    makes_copy(x);                 // x would move into the function, but i32 is Copy, so it's okay to still
                                   // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

Returning values can also transfer ownership.

```rust
fn main() {
    let s1 = gives_ownership();        // gives_ownership moves its return value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back which also moves its return value
                                       // into s3

} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}
```

Rust does let us return multiple values using a tuple:

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
```

A _reference_ is like a pointer in that it’s an address we can follow to access the data stored at that address; that
data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a
particular type for the life of that reference.

Here is how you would define and use a `calculate_length` function that has a reference to an object as a parameter
instead of taking ownership of the value:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.
```

References are immutable by default just like variables. So we need to explicitly make them mutable if needed:

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other
references to that value. This code that attempts to create two mutable references to `s` will fail:

```rust
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```

The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a
very controlled fashion. It’s something that new Rustaceans struggle with because most languages let you mutate whenever
you’d like. The benefit of having this restriction is that Rust can prevent data races at compile time. A _data race_ is
similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

We can use curly brackets to create a new scope, allowing for multiple mutable references, just not _simultaneous_ ones:

```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
```

Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:

```rust
    let mut s = String::from("hello");

    let r1 = &s;     // no problem
    let r2 = &s;     // no problem
    let r3 = &mut s; // BIG PROBLEM
                     // compiler error: cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("{}, {}, and {}", r1, r2, r3);
```

Users of an immutable reference don’t expect the value to suddenly change out from under them! However, multiple
immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s
reading of the data.

Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is
used. For instance, this code will compile because the last usage of the immutable references, the `println!`, occurs 
before the mutable reference is introduced:

```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```

## The Slice Type

_Slices_ let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice
is a kind of reference, so it does not have ownership.

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to
`ending_index` minus `starting_index`. So, in the case of `let world = &s[6..11];`, `world` would be a slice that
contains a pointer to the byte at index `6` of `s` with a length value of `5`.

With Rust’s `..` range syntax, if you want to start at index 0, you can drop the value before the two periods. In other
words, these are equal:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

By the same token, if your slice includes the last byte of the `String`, you can drop the trailing number. That means
these are equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

You can also drop both values to take a slice of the entire string. So these are equal:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

## Structs

A `struct`, or `structure`, is a custom data type that lets you package together and name multiple related values that
make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data
attributes. `Structs` and `enums` are the building blocks for creating new types in your program’s domain to take full
advantage of Rust’s compile-time type checking.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("hesamurai"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

As with any expression, we can construct a new instance of the struct as the last expression in the function body to
implicitly return that new instance.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand syntax
        email, // field init shorthand syntax
        sign_in_count: 1,
    }
}
```

It’s often useful to create a new instance of a struct that includes most of the values from another instance, but 
changes some. You can do this using struct update syntax.

```rust
fn main() {
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // remaining fields not explicitly set should have the same value as the fields in the given instance
    };
}
```

### Tuple Structs
Rust also supports structs that look similar to tuples, called `tuple structs`.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

### Unit-Like Structs

You can also define structs that don’t have any fields! Unit-like structs can be useful when you need to implement a
trait on some type but don’t have any data that you want to store in the type itself.

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

## Enums

Enums allow you to define a type by enumerating its possible variants. Where structs give you a way of grouping 
together related fields and data, like a `Rectangle` with its `width` and `height`, enums give you a way of saying a 
value is one of a possible set of values.

```rust
enum IpAddrKind {
    V4,
    V6,
}

// We can create instances of each of the two variants of IpAddrKind like this:
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

Representing the same concept using just an enum is more concise: rather than an enum inside a struct, we can put 
data directly into each enum variant. This new definition of the `IpAddr` enum says that both `V4` and `V6` variants 
will have associated `String` values. Here, it’s also easier to see another detail of how enums work: the name of 
each enum variant that we define also becomes a function that constructs an instance of the enum:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts 
of associated data. Version four IP addresses will always have four numeric components that will have values between 
0 and 255. If we wanted to store `V4` addresses as four u8 values but still express `V6` addresses as one `String` 
value, we wouldn’t be able to with a struct. Enums handle this case with ease:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

Let’s look at another example of an enum which has a wide variety of types embedded in its variants:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

The following structs could hold the same data that the preceding enum variants hold:

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

But if we used the different structs, each of which has its own type, we couldn’t as easily define a function to 
take any of these kinds of messages as we could with the `Message` enum defined earlier which is a single type. 
We’re also able to define methods on enums. Here’s a method named `call` that we could define on our Message enum:

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call(); // The body of the method would use self to get the value that we called the method on.
```

### The Option Enum and Its Advantages Over Null Values

Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. 
This enum is `Option<T>`, and it is defined by the standard library as follows:

```rust
enum Option<T> {
    None,
    Some(T),
}

let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```

Above, The type of `some_number` is `Option<i32>`. The type of `some_char` is `Option<char>`, which is a different type. 
Rust can infer these types because we’ve specified a value inside the `Some` variant. For `absent_number`, Rust requires 
us to annotate the overall `Option` type: the compiler can’t infer the type that the corresponding `Some` variant will 
hold by looking only at a `None` value. The `Option<T>` enum has a large number of methods that are useful in a variety
of situations; [Read more](https://doc.rust-lang.org/std/option/enum.Option.html).

### The `match` Control Flow Construct

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### Patterns That Bind to Values

```rust
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

#### Catch-all Patterns and the `_` Placeholder

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (), // unit value (the empty tuple type) which means "do nothing"
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

### Concise Control Flow with `if let`

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
// the code above can be shortened to:
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

## Module System

### Package

A `package` is a bundle of one or more crates that provides a set of functionality:

- A package can contain as many binary crates as you like, but at most only one library crate.
- A package must contain at least one crate, whether that’s a library or binary crate.

In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by 
default. If you want to make an item like a function or struct private, you put it in a module. Items in a parent 
module can’t use the private items inside child modules, but items in child modules can use the items in their 
ancestor modules. This is because child modules wrap and hide their implementation details, but the child modules 
can see the context in which they’re defined. Rust does give you the option to expose inner parts of child modules’ 
code to outer ancestor modules by using the `pub` keyword to make an item public. The pub keyword on a module only 
lets code in its ancestor modules refer to it, not access its inner code. The privacy rules apply to structs, enums, 
functions, and methods as well as modules.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

We can construct relative paths that begin in the parent module, rather than the current module or the crate root, 
by using super at the start of the path.

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```
