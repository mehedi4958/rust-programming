//--------------------------------------
//          - Operators
//--------------------------------------

fn main() {
    // Arithmetic operators
    // +,-,*,/,%
    // Note: ^ is not for exponential in Rust, rather pow() is used.
    println!("Remainder after dividing 17 by 5 is: {}", 17 % 5);

    // Comparison operators
    // == --> Equality
    // != --> Inequality
    // >, <, >=, <= --> Relational
    let a = 10;
    let b = 20;
    println!(
        "a==b: {}, a!=b: {}, a>b: {}, a<b: {}, a>=b: {}, a<=b: {}",
        a == b,
        a != b,
        a > b,
        a < b,
        a >= b,
        a <= b,
    );

    // Logical operators
    // && --> And
    // || --> Or
    // ! --> Not
    let a = 10;
    let b = 20;

    if (a > 5) && (b < 25) {
        println!("Condition satisfied");
    } else {
        println!("Condition not satisfied");
    }

    // Assignment operators
    // += --> Add and assign
    // -= --> Subtract and assign
    // *= --> Multiply and assign
    // /= --> Divide and assign
    // %= --> Remainder and assign
    let mut _x = 5;
    _x += 5; // x = x + 5
    _x -= 5; // x = x - 5
    _x *= 5; // x = x * 5
    _x /= 5; // x = x / 5
    _x %= 5; // x = x % 5

    // Bitwise operators
    // & --> AND
    // | --> OR
    // ^ --> XOR
    // ~ --> NOR
    // << --> Left Shift
    // >> --> Right Shift
    let x: u8 = 4;
    println!("{}", x & x); // ==> 0000_0100 & 0000_0100 = 0000_0100
    let _y:u8 = 4 << 1; // ==> 0000_0100 -> 0000_1000
}
