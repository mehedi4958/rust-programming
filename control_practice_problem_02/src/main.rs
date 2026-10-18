// Problem 2:
/*
Write a program to find the sum of natural numbers below a given number n, where number n will be defined in the program and can be any unsigned integer.
The sum should only include numbers that are multiples of either 3 or 5.
For example, if the n = 20, the multiples of 3 are 3, 6, 9, 12, 15, 18, and the multiples of 5 are 5, 10, and 15.
The program should be generalized and should give correct output for any value of number n.

Please note that the value of 15 will be considered only once since it is a multiple of both 3 and 5.
The sum will be calculated as follows: 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18.

Write a program that takes the user input N, performs the necessary calculations, and outputs the sum.
*/

fn main() {
    let n: u32 = 20;

    /* Add your code below this line */
    let mut sum = 0;

    for number in 1..n {
        if number % 3 == 0 || number % 5 == 0 {
            sum = sum + number;
        }
    }

    println!("\n\n The sum of the multiples are = {sum}");
}

// Note: You may require the % operator which is used for computing the remainder
// after dividing a number by another number. It's commonly used to check for divisibility.
// Example: The statement 17 % 5 will result in a value of 2.
