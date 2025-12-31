fn main() {
    let food = "🍕 of 🍛";
    println!("{}", food.len());
    // can't do [0..3] because 🍛 is 4 bytes, and it would land into incomplete character
    let pizza_slice = &food[0..4];
    println!("{}", pizza_slice.len());
}
