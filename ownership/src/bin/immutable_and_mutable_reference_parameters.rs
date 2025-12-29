fn main() {
    let mut current_meal = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);
    current_meal = add_sugar_mut(current_meal);
    show_my_meal(&current_meal);
}

// meal: String
// mut meal: String
// meal: &String
// meal: &mut String

// borrows mutable reference and makes changes directly in memory on that adress
fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

// moves ownership to the function, has to return it
fn add_sugar_mut(mut meal: String) -> String {
    meal.push_str(", Add sugar");
    meal
}

fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}");
}
