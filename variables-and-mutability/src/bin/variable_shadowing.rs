use std::str::{FromStr};

fn main() {
    let grams_of_protein = "100.345";
    let grams_of_protein = f64::from_str(grams_of_protein).unwrap();
    let mut grams_of_protein: i32 = i32::from(grams_of_protein as i32);
    grams_of_protein += 5;
    println!("I need {} grams of protein daily.", grams_of_protein);
}
