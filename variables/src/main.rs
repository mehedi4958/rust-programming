//----------------------------------------
//          - Variables
//              - Definition
//              - Mutability
//              - Scope
//              - Shadowing
//          - Constants
//----------------------------------------
fn main() {
    // Variables Definition
    let x: i16 = 10;
    println!("x is: {x}");

    // Mutability
    let mut y = 5;
    y = 10;

    // Scopes
    {
        let z = 20;
    }
    // let s = z; // error. z is available in different scope

    // Shadowing
    // allows to declare a variable with same name as a previous one
    // effectively overriding the previous binding within the given scope.
    let t = 10;
    let t = t + 10;
    println!("t is: {t}");

    let u = 3;
    let u = 3.0;

    let v = 30;
    {
        let v = 40;
        println!("inner v is: {v}"); // 40
    }
    println!("v is: {v}"); //30

    // Constants
    const MAX_VALUE: u32 = 100;
}
