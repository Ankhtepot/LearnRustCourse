fn main() {
    let mut name = String::from("Boris");
    describe_string(&name);

    name.push_str(" Pask");
    describe_string(&name);

    name.push_str("haver");
    describe_string(&name);
}

fn describe_string(input: &String) {
    println!("||****************************************||");
    println!("Description of: {input}");
    println!("Length: {}, Capacity: {}", input.len(), input.capacity());
    println!("Pointer: {:p}", input.as_ptr());
}
