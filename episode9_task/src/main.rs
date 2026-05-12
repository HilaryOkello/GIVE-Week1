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
