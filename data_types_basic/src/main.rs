//---------------------------------
//          - Basic Data Types
//              - Integers
//              - Floats
//              - Chars
//              - Boolean
//---------------------------------
fn main() {
    // Unsigned integer
    let unsigned_number: u8 = 5; // u16, u32, u64, u128

    // Signed integer
    let signed_number: i8 = 5; // i16, i32, i64, i128

    // Floating point number
    let float_number: f32 = 5.0; // f64

    // Platform specific integers
    let arch_1: usize = 5;
    let arch_2: isize = 5;

    // Character
    let char: char = 'a';

    // Boolean
    let b: bool = true;

    // Type Aliasing
    type Age = u8;
    let peter_age: Age = 25;

    // Type Conversion
    let a = 10;
    let b = a as f64;
}
