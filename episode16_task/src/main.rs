// Try This
//
// Program requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// Use a struct containing the student's name and locker assignment
// The locker assignment should use an Option<i32>

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
