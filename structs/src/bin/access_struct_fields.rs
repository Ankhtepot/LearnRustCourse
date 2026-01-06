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

    println!("Cofee name: {}", &mocha.name);
    let price = &mocha.price;

    println!("Today coffee is {} for price of {}. And it is {}."
             , &mocha.name, price, if mocha.is_hot { String::from("hot") } else { String::from("cold") })
}