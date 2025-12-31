fn main() {
    // Define a `cereals` array with 5 heap Strings
    //   - Cookie Crisp
    //   - Cinnamon Toast Crunch
    //   - Frosted Flakes
    //   - Cocoa Puffs
    //   - Captain Crunch
    let mut cereals: [String; 5] = ["Cookie Crisp", "Cinnamon Toast Crunch", "Frosted Flakes", "Cocoa Puffs", "Captain Crunch"]
        .map(String::from);

    // Declare a `first_two` variable that extracts a slice
    // of the first two cereals. Print the slice.
    let first_two = &cereals[..2];
    println!("{:?}", first_two);

    // Declare a `mid_three` variable that extracts a slice
    // of the middle three cereals (Cinnamon Toast Crunch
    // up to and including Cocoa Puffs). Print the slice.
    let mid_three = &cereals[1..4];
    println!("{:?}", mid_three);

    // Declare a `last_three` variable that extracts a slice
    // of the last three cereals. Print the slice.
    let last_three = &mut cereals[2..];
    println!("{:?}", last_three);

    // Using the `last_three` slice, target the last element
    // ("Captain Crunch") and replace it with "Lucky Charms".
    // Print the complete `cereals` array.
    last_three[2] = String::from("Lucky Charms");
    println!("{:?}", cereals);

    // Declare a `cookie_crisp` variable. Make it a reference
    // to the "Cookie Crisp" String (in other words, a &String).
    let cookie_crisp = &cereals[0];

    // Declare a `cookie` variable that extracts a slice of
    // the text "Cookie" from the String. Print the slice.
    let cookie = &cookie_crisp[..6];
    println!("{:?}", cookie);

    // Declare a `cocoa_puffs` variable. Make it a reference
    // to the "Cocoa Puffs" String (in other words, a &String).
    let cocoa_puffs = &cereals[3];

    // Declare a `puffs` variable that extracts a slice of
    // the text "Puffs" from the String. Print the slice.
    let puffs = &cocoa_puffs[6..];
    println!("{:?}", puffs);
}
