fn main() {
    let action_hero = String::from("Arnold Schwarzenegger");

    let break_index = action_hero.find(' ');

    if break_index.is_none() {
        println!("No space found in action hero name");
        return;
    }

    let first_name = &action_hero[..break_index.unwrap()];
    println!("His first name is {first_name}.");

    let last_name = &action_hero[break_index.unwrap() + 1..];
    println!("His first name is {last_name}.");

    let full_name = &action_hero[..];
    println!("His full name is {full_name}.");
}
