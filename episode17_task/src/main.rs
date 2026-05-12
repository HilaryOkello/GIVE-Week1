fn coordinate() -> (i32, i32) {
    (3, 7)
}

fn main() {
    let (x, y) = coordinate();

    if y > 5 {
        println!("y is greater than 5");
    } else if y < 5 {
        println!("y is less than 5");
    } else {
        println!("y is equal to 5");
    }

    println!("x: {}, y: {}", x, y);
}
