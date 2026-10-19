//--------------------------------------------------
//          - Associativity
//          - Explicit Boolean in Conditionals
//          - Operator Overloading
//--------------------------------------------------

fn main() {
    // Associativity
    // Arithmetic operator are left associative
    let _x = 8 / 4 / 2; // (8/4)/2 = 1

    // Assignment operators a right associative
    // =, +=, -=, *=, /=, %=, &=, |=, ^=, ~=, <<=, >>=

    // Explicit Boolean in Conditionals
    let x = 0;
    if x != 0 {}

    // Operator Overloading
    let _a = 10 + 20; // + --> Addition
    let s = String::from("1") + "2"; // + --> Concatenation
    println!("{}", s);
}
