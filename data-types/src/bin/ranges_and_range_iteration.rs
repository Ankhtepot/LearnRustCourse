fn main() {
    let month_days = 1..31;
    println!("{month_days:?}");

    let month_days = 1..=31;
    println!("{month_days:?}");

    for number in month_days {
        print!("{number}");
    }
    println!();
    // Recreating again because InclusiveRange took the iterator
    let month_days = 1..=31;
    println!("Debug: {month_days:#?}");

    let letters = 'a'..'f';
    // OR Inclusive
    // let letters = 'a'..='f';

    for letter in letters {
        println!("{letter}");
    }

    let colors = ["Red", "Green", "Yellow"];

    for color in colors {
        println!("{color} is a great color!");
    }
}
