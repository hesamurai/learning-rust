fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // return x + 1; // would work, too
    // x + 1; // would NOT work since it is a statement, not expression
}
