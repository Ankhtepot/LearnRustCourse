fn main() {
    open_store("Brooklyn");
    bake_pizza(20, "pepperoni");
    let mut total_profit = 0.0;
    total_profit += swim_in_profit(12.64);
    total_profit += swim_in_profit(25.84);
    total_profit += swim_in_profit(164.98);
    open_store("Queens");
    bake_pizza(15, "mushroom");
    write_total_profit(total_profit);
}

fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {neighborhood}");
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizzas");
}

fn swim_in_profit(profit: f64) -> f64 {
    println!("Today's profit:");
    println!("{profit}. So much $$$, so little time.");

    profit
}

fn write_total_profit(total:f64) {
    println!("Total profit for the day: {total:.2}");
}
