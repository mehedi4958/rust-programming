// Problem 4: Add a type alias for Book so that we are able to store the information.

fn main() {
    type Book = (String, String, i32);// Tuple alias defined with data types

    let book1: Book = (
        String::from("Rust Programming Language"),
        String::from("RUST Community"),
        2010,
    );
    println!(
        "Book name: {}, Author: {}, Year {}",
        book1.0, book1.1, book1.2
    );

    let book2: Book = (
        String::from("Rust by Example"),
        String::from("Steve Klabnik and Carol Nichols"),
        2015,
    );
    println!(
        "Book name: {}, Authors: {}, Year {}",
        book2.0, book2.1, book2.2
    );
}
