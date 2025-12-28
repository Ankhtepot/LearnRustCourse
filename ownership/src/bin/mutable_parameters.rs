fn main() {
    let mut burger = String::from("Burger");
    add_fries_copy(&burger);
    add_fries_borrow(&mut burger);
    add_fries(burger);
}

fn add_fries(mut meal: String) {
    meal.push_str(" and Fries");
    println!("{meal}");
}

fn add_fries_borrow(meal: &mut String) {
    println!("|**********************************|");
    println!("Borrowed meal change:");
    meal.push_str(" ADDED");
    println!("{meal}");
}

fn add_fries_copy(meal: &String) {
    println!("|**********************************|");
    println!("Owned meal copy change:");
    let mut copy = String::from(meal);
    copy.push_str(" COPIED");
    println!("{copy}");
}
