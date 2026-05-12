// Try This
//
// Program requirements:
// * Print the flavor of a drink and its fluid ounces
//
// Notes:
// Use an enum to create different flavors of drinks
// Use a struct to store drink flavor and fluid ounce information
// Use a function to print out the drink flavor and ounces
// Use a match expression to print the drink flavor

enum Flavor {
    Blackcurrant,
    Passion,
    Pineapple,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    let flavor_name = match drink.flavor {
        Flavor::Blackcurrant => "Blackcurrant",
        Flavor::Passion => "Passion",
        Flavor::Pineapple => "Pineapple",
    };
    println!("Flavor: {}, Fluid oz: {}", flavor_name, drink.fluid_oz);
}

fn main() {
    let drink = Drink {
        flavor: Flavor::Blackcurrant,
        fluid_oz: 12.0,
    };
    print_drink(drink);
}
