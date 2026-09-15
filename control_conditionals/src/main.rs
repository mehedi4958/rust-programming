//-------------------------------------
//          Conditionals
//              - if, else
//              - if, else if ladder
//              - match
//--------------------------------------

fn main() {
    // if, else
    let num = 40;

    if num < 50 {
        println!("The number is less than 50");
    } else {
        println!("The number is greater than or equal to 50");
    }

    // if, else if ladder
    let mark = 95;
    let grade = if mark >= 90 {
        'A'
    } else if mark >= 80 {
        'B'
    } else if mark >= 70 {
        'C'
    } else {
        'F'
    };

    println!("The grade is {grade}");

    // match
    let mark = 85;
    let grade = match mark {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        _ => 'F',
    };

    println!("The matched grade is {grade}");
}
