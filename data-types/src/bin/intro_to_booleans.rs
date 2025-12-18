fn main() {
    let is_handsome: bool = true;
    let is_silly: bool = false;

    println!("Handsome: {is_handsome}. Silly: {is_silly}");

    let age: i32 = -40;
    let is_young = age < 35;
    println!("Is young: {is_young}");
    println!("Positive age ({age}) with \".isP_positive\": {}\nNegative age ({age}) with \".is_negative\": {}",
             age.is_positive(), age.is_negative());
}
