// Problem 2: Refactor the code by taking rid of the variables x and y in main.
// The main should only consist of a print statement.

// Further Explanation: Rewrite the code in a way that produce the same outcome
// as the original code, but without using any variables.

fn double(x: i32) -> i32 {
    x * 2
}

fn triple(x: i32) -> i32 {
    x * 3
}

fn main() {
    // let x = triple(double(5));
    // let y = triple(x);
    println!("Answer: {}", triple(triple(double(5))));
}
