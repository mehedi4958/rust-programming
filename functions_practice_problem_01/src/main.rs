// Problem 1: Add the following missing functions to the code.

// 1. add_3(x): This function should add three to the input variable ‘x’ and the return the resultant value.
// 2. add_5(x): This function should add five to the input variable ‘x’ and the return the resultant value.
// 3. times(x,y): This function should compute the multiplication of the inputs ‘x’ and ‘y’ and return the resultant value.

fn add_3(x: i32) -> i32 {
    x + 3
}

fn add_5(y: i32) -> i32 {
    y + 5
}

fn times(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    let x = 3;
    let y = 4;
    println!(
        "The result of x+3 times y+5 is {}",
        times(add_3(x), add_5(y))
    );
}
