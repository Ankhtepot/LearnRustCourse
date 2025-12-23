fn main() {
    let number = 37;

    //"value" and "x" are arbitrary name
    //unreachable!() macro indicates that this point in the code should never be reached
    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        x if x % 2 != 0 => println!("{x} is an odd number"),
        _ => unreachable!(),
    }

    match number {
        1 | 3 | 5 | 7 | 9 => println!("{number} is a single-digit odd number"),
        2 | 4 | 6 | 8 => println!("{number} is a single-digit even number"),
        _ => println!("{number} is not a single-digit number"),
    }
}
