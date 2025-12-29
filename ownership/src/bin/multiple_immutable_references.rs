fn main() {
    let mut car = String::from("Red");
    let ref1 = &car;
    let ref2 = &car;
    // let ref3 = &mut car; // Shows an error, cant borrow it when it's used as immutable
    println!("{ref1} and {ref2} and {}", &car);
    let ref3 = &mut car; // Here it's OK, because immutable references are not used anymore
}
