//-------------------------------------
//          Comments
//          More on Printing
//-------------------------------------

fn main() {
    // Comments

    // This line is a comment
    // This is a 2nd comment

    /* This is a
    multi-lin
    comment
    */

    // More on Printing
    print!("This is a print command.");
    print!("This is another print command which will be printed in the same line.");

    // Escape Sequences
    // \escape_sequence_character

    /*
        \n --> New line
        \t --> Tap space
        \r --> Carriage return
        \" --> Double quote
        \\ --> Backward slash
    */

    println!("\n\nSubsequent printing on the next line.");
    println!("\tA tab space at the start.");

    println!("This will be overwritten \rThis will replace on top of the overwritten line.");
    println!("Printing double quotes \"\", printing back slash\\.");

    // Positional Arguments
    println!(
        "I am doing {2} for {1} years and I {0} it.",
        "like", 20, "programming"
    );
    println!(
        "{language} is a system programming language which is cool to {activity} with",
        activity = "code",
        language = "Rust",
    );
}
