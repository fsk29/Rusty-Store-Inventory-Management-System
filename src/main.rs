use std::io;

mod inventory;

fn main() {
    println!("Welcome to Rusty Store Inventory Management System");

    // Simulated user login (placeholder)
    login();

    // Initialize inventory
    let mut inventory = inventory::Inventory::new();

    loop {
        println!("Menu:");
        println!("1. Add Product");
        println!("2. List Products");
        println!("3. Exit");
        println!("Please enter your choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => add_product(&mut inventory),
            "2" => list_products(&inventory),
            "3" => {
                println!("Exiting the program.");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn login() {
    // Implement user authentication here (placeholder)
    println!("User authentication placeholder");
}

fn add_product(inventory: &mut inventory::Inventory) {
    println!("Enter product name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Enter product description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read line");

    println!("Enter product price:");
    let mut price_str = String::new();
    io::stdin().read_line(&mut price_str).expect("Failed to read line");
    let price: f64 = match price_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid price, defaulting to 0");
            0.0
        }
    };

    println!("Enter product quantity:");
    let mut quantity_str = String::new();
    io::stdin().read_line(&mut quantity_str).expect("Failed to read line");
    let quantity: u32 = match quantity_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid quantity, defaulting to 0");
            0
        }
    };

    let product = inventory::Product::new(name.trim().to_string(), description.trim().to_string(), price, quantity);
    inventory.add_product(product);

    println!("Product added successfully.");
}

fn list_products(inventory: &inventory::Inventory) {
    println!("Products in Inventory:");
    inventory.list_products();
}