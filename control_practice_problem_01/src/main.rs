// Problem 1:
/*
Write a program to find the difference between the square of the sum and the sum of the squares of the first n numbers.
The number n will be defined in the program and is a positive integer greater then or equal to 1.

For example, if n = 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.
Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.
Finally, calculate the difference as 225 - 55 = 170.

The program should be however generalized and should work for any value of variable N.
*/

fn main() {
    let n: i32 = 5;
    // let mut square_of_sum = 0;
    let mut sum_of_squares = 0;

    /* Complete the code after this line */
    let mut sum_of_numbers = 0;
    for number in 1..=n {
        sum_of_numbers = sum_of_numbers + number;
        sum_of_squares = sum_of_squares + number.pow(2);
    }
    let square_of_sum = sum_of_numbers.pow(2);
    let difference = square_of_sum - sum_of_squares;
    println!(
        "The difference of the square_of_sum and sum_of_squares for n = {} is {}",
        n, difference
    );
}

// Note: The pow() method in Rust is used to raise a number to an integer power.
// Example: If we want to compute 2 raised to the power 3, then the syntax
// will be 2.pow(3).
// If we want to compute a variable raise to the power of some value,
// then the following syntax can be used.
// let number: u32 = 5;
// let square = number.pow(2); // This computes square of the variable number
