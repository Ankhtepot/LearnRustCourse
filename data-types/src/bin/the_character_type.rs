fn main() {
    const FIRST_INITIAL: &str = "First Initial: ";
    const EMOJI: &str = "Emoji: ";

    let first_initial = 'B';
    let emoji = '🎧';

    const CHARACTERS: [char; 6] = ['A', 'g', '5', ' ', '\n', '🎉'];

    println!(
        "{FIRST_INITIAL}is alphabetic: {}\n{EMOJI}is alphabetic: {}",
        first_initial.is_alphabetic(),
        emoji.is_alphabetic()
    );

    println!("{FIRST_INITIAL}is uppercase: {}\n{EMOJI}is uppercase: {}", first_initial.is_uppercase(), emoji.is_uppercase());
    println!("{FIRST_INITIAL}is lowercase: {}\n{EMOJI}is lowercase: {}", first_initial.is_lowercase(), emoji.is_lowercase());

    describe_characters(&CHARACTERS);
}

fn describe_characters(chars: &[char]) {
    for &c in chars {
        describe_character(c);
        println!();
    }
}

fn describe_character(c: char) {
        println!("Describing character: \"{c}\"");
        println!("\"{c}\" is an alphabetic character {}.", c.is_alphabetic());
        println!("\"{c}\" is an uppercase character {}.", c.is_uppercase());
        println!("\"{c}\" is a lowercase character {}.", c.is_lowercase());
        println!("\"{c}\" is a whitespace character {}.", c.is_whitespace());
        println!("\"{c}\" is a numeric character {}.", c.is_numeric());
        println!("\"{c}\" is a ascii character {}.", c.is_ascii());
        println!("\"{c}\" is a alphanumeric character {}.", c.is_alphanumeric());
}