fn main() {
    let mut my_array = [10, 15, 20, 25, 30];
    println!("My array: {:?}", my_array);
    let my_slice: &mut [i32] = &mut my_array[2..4];
    println!("My slice: {:?}", my_slice);

    my_slice[0] = 100;
    println!("My slice after changing an element: {:?}", my_slice);
    // Changes over mutable references can affect the original array
    println!("My array: {:?}", my_array);
}
