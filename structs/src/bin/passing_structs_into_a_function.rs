struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}

fn main() {
    let mut mocha = make_coffee(String::from("Mocha"), 4.99, true);
    print_beverage(&mocha);
    drink_coffee(&mut mocha);
    print_beverage(&mocha);
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}

fn drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my delicious {}", coffee.name);
    coffee.is_hot = false;
    coffee.price = 1.99;
}

fn print_beverage(beverage: &Coffee) {
    let is_hot = if beverage.is_hot { "hot" } else { "cold" };
    println!("My {} this morning cost {}. It is now {is_hot}.", beverage.name, beverage.price);
}
