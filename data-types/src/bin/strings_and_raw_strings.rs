fn main() {
    println!("Dear Emily,\nHow have you been?");
    println!("\tOnce upon a time");
    println!("Juliet said \"I love you Romeo\"");

    // let filepath = "C:\My Documents\new\videos"; // <-- This will cause an error (\M, \v are not valid escape sequences)
    let filepath = "C:\\My Documents\\new\\videos"; // <-- This works via escape sequences
    // let filepath = r"C:\My Documents\new\videos"; // <-- This also works via raw string literal
    println!("{filepath}");
}
