fn main() {
    let oranges = String::from("Oranges");
    print_my_value(oranges); // let value = oranges;
    // this is not possible, oranges moved ownership when calling the method above
    // println!("{oranges} is now invalid");

    let apples = String::from("Apples");
    print_referenced_value(&apples);
    println!("After the function: {}", apples);
}

fn print_my_value(value: String) {
    println!("Referenced by value, ownership move to here:");
    println!("Your value is {value}");
}

fn print_referenced_value(value: &String) {
    println!("referenced value");
    println!("Your value is {value}");
    println!("|****************************|")
}
