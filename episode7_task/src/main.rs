// Try This
//
// Program requirements:
// * Use a match expression with a boolean
// * Use a match expression with a number
//
// Notes:
// Use a match expression to print whether a boolean is true or false
// Use a match expression to print the name of a number

fn main() {
    let my_bool = true;

    match my_bool {
        true => println!("it's true"),
        false => println!("it's false"),
    }

    let my_number = 2;

    match my_number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}
