fn main() {
    let seasons = ["Spring", "Summer", "Fall", "Winter"];

    println!("{}", 5);
    println!("{}", 3.14);
    println!("{}", true);
    println!("\":? format\"{seasons:?}");
    println!();
    println!("\":#? format\"{seasons:#?}");
    println!();
    println!("\"dbg!(seasons) call\"");
    dbg!(seasons);
}
