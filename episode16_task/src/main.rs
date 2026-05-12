struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let students = vec![
        Student { name: String::from("Amara"), locker: Some(204) },
        Student { name: String::from("Kofi"), locker: None },
        Student { name: String::from("Zuri"), locker: Some(317) },
    ];

    for student in &students {
        match student.locker {
            Some(number) => println!("{} is assigned locker {}", student.name, number),
            None => println!("{} has no locker assigned", student.name),
        }
    }
}
