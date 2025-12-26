fn main() {
    let person = String::from("Boris");
    println!("My name is {person}");

    let genius = &person; // owner is still "person"
    let third = person.clone(); // creates new instance, owner is "third"

    println!("My name is {person} ({genius}, {third})");
}
