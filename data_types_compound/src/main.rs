use core::num;

//-------------------------------------
//          - Compound Data Types
//              - &str and String
//              - Arrays
//              - Vectors
//              - Tuples
//              - Empty Tuples
//-------------------------------------
fn main() {
    // Strings
    let fixed_string = "Fixed length string";
    let mut flexible_string = String::from("This string will grow");
    flexible_string.push('s');

    // Arrays
    let mut array_1: [i32; 5] = [5, 6, 7, 8, 9]; // we can remove the type annotation and size
    let num = array_1[3];

    // prints the entire array
    println!("{:?}", array_1); // :? is called the debug format specifier

    let array_2 = [0; 10]; // 0 is the default value and 10 is the size of the array

    // Vectors
    let vec_1: Vec<i32> = vec![5, 6, 7, 8, 9];
    let num = vec_1[3];

    // Tuples
    let my_info = ("Salary", 60000, "Age", 33);
    let salary_value = my_info.1;

    // Destructuring tuples
    let (salary, salary_value, age, age_value) = my_info;

    // Empty Tuple
    let unit = ();
}
