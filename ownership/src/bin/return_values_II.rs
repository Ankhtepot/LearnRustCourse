fn main() {
    let mut current_meal = String::from("Meat");
    current_meal = add_flour(current_meal);
    current_meal = add_sugar(current_meal);
    current_meal = add_salt(current_meal);
    // add_salt();
    println!("{current_meal}");
}

fn add_flour(mut meal: String) -> String {
    meal.push_str(", Add flour");
    meal
}

fn add_sugar(mut meal: String) -> String {
    meal.push_str(", Add sugar");
    meal
}

fn add_salt(mut meal: String) -> String {
    meal.push_str(", Add salt");
    meal
}
