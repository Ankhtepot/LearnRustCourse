struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn main() {
    let name = String::from("Latte");
    let coffee: Coffee = make_coffee(name, 4.99, true);
    let beverage = Coffee {price: 3.99, name: String::from("Bier"), is_hot: false};
    print_beverage(coffee);
    print_beverage(beverage);
}

fn print_beverage(beverage: Coffee) {
    println!("My {} this morning cost {}. It is {} that it was hot.", beverage.name, beverage.price, beverage.is_hot);
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name: name,
        price: price,
        is_hot: is_hot,
    }
}
