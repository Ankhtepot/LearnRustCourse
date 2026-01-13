fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

fn main() {
    let t1 = make_tuple(5, "hello");
    println!("{:?}", t1);
    let t1 =make_tuple(5, 13);
    println!("{:?}", t1);
    let t1 = make_tuple(true, 3.14);
    println!("{:?}", t1);
    let t1 = make_tuple(true, false);
    println!("{:?}", t1);
    let t1 = (25, true); // This works too, of course, it's just to demonstrate
    // multiple generic type arguments.
    let t1 =
    println!("{:?}", t1);
}
