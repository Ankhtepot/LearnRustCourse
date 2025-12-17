fn main() {
    let pi: f64 = 3.1415926535897932384;
    // :.4 limits the number of decimal places to 4:
    // - : is used to introduce the format specifier
    // - .4 indicates that we want 4 digits after the decimal point
    println!("The current value of pi is {:.4}", pi);
    // Alternatively, you can use the named argument feature of Rust's formatting:
    println!("The current value of pi is {pi:.4}");
}
