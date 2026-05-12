use std::collections::HashMap;
use std::io;

fn get_input() -> Option<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    let trimmed = input.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

fn add_bill(bills: &mut HashMap<String, f64>) {
    println!("Enter bill name:");
    let name = match get_input() {
        Some(n) => n,
        None => return,
    };

    println!("Enter amount:");
    let amount: f64 = match get_input() {
        Some(a) => match a.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid amount.");
                return;
            }
        },
        None => return,
    };

    bills.insert(name, amount);
    println!("Bill added.");
}

fn view_bills(bills: &HashMap<String, f64>) {
    if bills.is_empty() {
        println!("No bills.");
        return;
    }
    for (name, amount) in bills {
        println!("{}: ${:.2}", name, amount);
    }
}

fn main() {
    let mut bills: HashMap<String, f64> = HashMap::new();

    loop {
        println!("\n-- Bill Manager --");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Quit");
        println!("Choose:");

        match get_input().as_deref() {
            Some("1") => add_bill(&mut bills),
            Some("2") => view_bills(&bills),
            Some("3") => break,
            _ => println!("Invalid choice."),
        }
    }
}
