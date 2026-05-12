fn main() {
    let numbers = vec![10, 20, 30, 40];

    for num in &numbers {
        if *num == 30 {
            println!("thirty");
        } else {
            println!("{}", num);
        }
    }

    println!("Total elements: {}", numbers.len());
}
