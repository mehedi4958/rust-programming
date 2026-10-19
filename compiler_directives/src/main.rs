//-------------------------------------------
//          - Compiler Directives
//-------------------------------------------

#![allow(unused_variables)] //allows on the whole file

#[allow(dead_code)] // allows for a single instance
fn square(x: i32) -> i32 {
    let temp = 42;
    x * x
}

fn main() {
    let i = 10;
    let s = String::from("Hello there");
}
