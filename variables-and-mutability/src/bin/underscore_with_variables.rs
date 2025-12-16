fn main() {
    let apples = 50;
    let oranges = 14 + 6;
    let _fruits = apples + oranges;

    println!(
        "This year, my garden has {0} apples and {1} oranges. I can't believe I have {0} apples",
        apples, oranges
    );

    // My solution:
    let apples: i32 = 50;
    let oranges: i32 = 14 + 6;
    // For when you want to silence the warning about an unused variable "_" on start.
    let _unused_variable: &str = "I like to drink water";

    println!("Positional arguments:");
    println!("Apples: {0}\nOranges: {1}\nTotal fruits: {2}\n", apples, oranges, apples + oranges);
    println!("Positional arguments practically (apples and oranges swapped):");
    println!("Apples: {1}\nOranges: {0}\nTotal fruits: {2}\n", apples, oranges, apples + oranges);
    println!("When arguments are in sequential order:");
    println!("Apples: {}\nOranges: {}\nTotal fruits: {}\n", apples, oranges, apples + oranges);
    println!("Interpolation (kind of... operations cant be interpolated in this version yet):");
    println!("Apples: {apples}\nOranges: {oranges}\nTotal fruits: {0}", apples + oranges);
}
