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
