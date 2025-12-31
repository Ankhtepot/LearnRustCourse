fn main() {
    let values = [4, 8, 15, 16, 23, 42];

    let my_slice = &values[..4];
    println!("{my_slice:?}");

    let my_slice = &values[2..4];
    println!("{my_slice:?}");

    let my_slice = &values[2..];
    println!("{my_slice:?}");

    let my_slice = &values[..];
    println!("{my_slice:?}");
    // Notice the difference in type, with slice brackets its dynamic array,
    // reference creates a bounded array
    let my_slice = &values;
    println!("{my_slice:?}");
}
