fn even_or_odd(number: i32) {
    let mut result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {result}");

    /*
    Won't work in Rust, no ternary operator in Rust
    let result = number % 2 == 0 ? "even" : "odd";
    println!("The number is {result}");
    */
}

fn main() {
    even_or_odd(17);
    even_or_odd(100);
}
