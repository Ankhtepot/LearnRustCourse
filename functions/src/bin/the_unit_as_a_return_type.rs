fn main() {
    let result = mystery();
    println!("The result of mystery is: {:?}", result);
}

fn mystery() -> (i32, &'static str) {
    println!("Hello there");
    (64, "some text")
}
