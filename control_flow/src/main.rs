//-----------------------------------
//          - Control Flow
//              - loops
//              - for loops
//              - while loops
//-----------------------------------

fn main() {
    // Loops
    'outer: loop {
        loop {
            println!("Simple loop");
            break 'outer;
        }
    }

    let a = loop {
        break 5;
    };
    println!("value is {a}");

    // for loops
    let vector = vec![45, 30, 85, 90, 41, 39];

    for i in vector {
        println!("{i} ");
    }

    // Compound Data Types versus Collections
    // In compound data types size and structure is known at compile time
    // the size can grow or shrink at runtime in a collection

    // while loops
    let mut number = 0;

    while number < 10 {
        println!("{number}");
        number += 1;
    }
}
