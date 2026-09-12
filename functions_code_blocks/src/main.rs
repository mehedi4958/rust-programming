//------------------------------------
//          - Functions
//          - Code Blocks
//------------------------------------

fn my_fn(s: &str) {
    println!("{s}");
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    println!("Computing multiplication");
    num1 * num2
}

fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 + num2, num1 - num2, num1 * num2)
}

// Expressions VS Statements
// An expression returns a valid value
// A statement returns a unit value

fn main() {
    my_fn("This is my functions");
    let str = "Function call with a variable";
    my_fn(str);
    let answer = multiplication(10, 15);
    let result = basic_math(10, 15);
    let (addition, subtraction, multiplication) = result;

    // Code Blocks

    let full_name = {
        let first_name = "Mehedi";
        let last_name = "Hasan";
        println!("{answer}");
        format!("{first_name} {last_name}")
    };
}
