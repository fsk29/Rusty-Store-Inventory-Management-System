pub struct Product {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
}
impl Product {
    pub fn new(name: String, description: String, price: f64, quantity: u32) -> Self {
        Product {
            name,
            description,
            price,
            quantity,
        }
    }
}
pub struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            products: Vec::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    pub fn list_products(&self) {
        for product in &self.products {
            println!(
                "Name: {}, Description: {}, Price: ${:.2}, Quantity: {}",
                product.name, product.description, product.price, product.quantity
            );
        }
    }
}