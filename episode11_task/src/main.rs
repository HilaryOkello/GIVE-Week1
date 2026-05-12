struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn print_quantity(item: &GroceryItem) {
    println!("Quantity: {}", item.quantity);
}

fn print_id(item: &GroceryItem) {
    println!("ID: {}", item.id);
}

fn main() {
    let item = GroceryItem {
        quantity: 5,
        id: 101,
    };
    print_quantity(&item);
    print_id(&item);
}
