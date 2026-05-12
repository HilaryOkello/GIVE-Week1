struct Person {
    name: String,
    age: u8,
    favorite_color: String,
}

fn print_person(person: &Person) {
    println!("Name: {}, Favorite color: {}", person.name, person.favorite_color);
}

fn main() {
    let people = vec![
        Person { name: String::from("Amara"), age: 8, favorite_color: String::from("orange") },
        Person { name: String::from("Zuri"), age: 14, favorite_color: String::from("cyan") },
        Person { name: String::from("Kofi"), age: 6, favorite_color: String::from("purple") },
        Person { name: String::from("Nia"), age: 11, favorite_color: String::from("pink") },
    ];

    for person in &people {
        if person.age <= 10 {
            print_person(person);
        }
    }
}
