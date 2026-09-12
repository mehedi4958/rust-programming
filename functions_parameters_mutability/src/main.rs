//-------------------------------------------------
//          - Mutability in Function parameters
//-------------------------------------------------

fn prints_number(mut x: i32) -> i32 {
    // x can also be mutable using Shadowing
    // let mut x = x;
    x = x + 2;
    println!("{x}");
    x
}

fn main() {
    let mut number = 10;
    let y = prints_number(number);
    //y = y + 3; // the mutability is not passed or received
}
