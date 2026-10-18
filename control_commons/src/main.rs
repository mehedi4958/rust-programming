//--------------------------------------------------
//          Common For Loop Syntaxes with Ranges
//--------------------------------------------------

fn main() {
    println!("Values in Ascending Order:");
    // Iterating over a range of values in ascending order
    for i in 0..5 {
        println!("i = {i}");
    }

    println!("Values in Ascending Order up to the Last Value:");
    // includes the last value
    for i in 0..=5 {
        println!("i = {i}");
    }

    // infinite loop
    // for i in 0.. {
    //     println!("i = {i}");
    // }

    println!("Values in Descending Order:");
    // Iterating over a range of values in descending order
    // start..end or start..=end only computed when end > start
    // The range is empty if end <= start
    // Need to use rev() to get descending order
    for i in (0..5).rev() {
        println!("i = {i}");
    }

    println!("Values in Descending Order up to the Last Value:");
    for i in (0..=5).rev() {
        println!("i = {i}");
    }

    println!("Values in Ascending Order with a step value of 2:");
    // Iterating with a step
    for i in (1..=10).step_by(2) {
        println!("i = {i}");
    }

    // Ranges and Step values cannot be floating numbers
    // for i in 0.0..10.0 is invalid

    println!("Values in Ascending Order with a step value of 0.2:");
    // for that we use different logics for different ranges or steps
    for i in 1..=5 {
        let value = i as f32 * 0.2;
        println!("i = {value}");
    }

    // Range is a valid data type in Rust Standard Library
    let r = 0..5;
    println!("Range Contains 3 is: {}", &r.contains(&3));
    println!("Range Count: {}", &r.count());

    // Iteration in Vector using for loop
    let pairs = vec![
        (1, "One"),
        (2, "Two"),
        (3, "Three"),
        (4, "Four"),
        (5, "Five"),
    ];
    for (num, word) in pairs {
        println!("{}: {}", num, word);
    }
}
