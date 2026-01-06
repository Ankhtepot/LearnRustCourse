fn main() {
    struct Cofee {
        price: f64,
        name: String,
        is_hot: bool,
    }
    
    let mocha: Cofee = Cofee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };
}