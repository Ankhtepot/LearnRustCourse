fn main() {
    let employee = ("Molly", 32, "Marketing");

    // let name = employee.0;
    // let age = employee.1;
    // let department = employee.2;

    // Deconstructing the tuple into individual variables
    let (name, age, department) = employee;
    println!("Name: {name}, age: {age}, department: {department}");
    println!("---");
    println!("Accessing tuple elements directly:");
    println!("Age: {1}, name: {0}, department: {2}", employee.0, employee.1, employee.2);
    println!("When using positional formatting:");
    println!("Name: {}, age: {}, department: {}", employee.0, employee.1, employee.2);
    println!("---");
    println!("Debug:");
    dbg!(employee);

}
