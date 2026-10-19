//---------------------------------------------
//          - Variable convention
//          - Unused variables
//          - Statics
//---------------------------------------------

fn main() {

    // Variable convention and Unused variable
    let _number_one = 45;
    let _x = 40_000;

    // Statics
    static WELCOME: &str = "Welcome to Rust.";
    const PI: f32 = 3.14;

    let a = PI; // no extra space is allocated for a in the memory
    let b = WELCOME; // additional space is allocated for b in the memory
}
