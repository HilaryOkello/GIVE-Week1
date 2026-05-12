// Try This
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// Use an enum with color names as variants
// Use a function to print the color name
// The function must use the enum as a parameter
// Use a match expression to determine which color name to print

enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Orange => println!("Orange"),
        Color::Yellow => println!("Yellow"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::Violet => println!("Violet"),
    }
}

fn main() {
    print_color(Color::Red);
    print_color(Color::Orange);
    print_color(Color::Yellow);
    print_color(Color::Green);
    print_color(Color::Blue);
    print_color(Color::Violet);
}
