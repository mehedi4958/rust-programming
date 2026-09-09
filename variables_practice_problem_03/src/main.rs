// Problem 3: Do not execute and answer if the code below will compile?

fn main() {
    let mut x1 = 40;
    let x2;
    x1 = x1 * 3;
    x2 = x1 - 2; // will compile as no prior binding made to x2
    println!("x1 is: {}, x2 is: {}", x1, x2);
}
