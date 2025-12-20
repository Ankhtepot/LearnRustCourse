fn main() {
    let mut seasons = ["Spring", "Summer", "Fall", "Winter"];

    println!("{}", seasons[2]);
    seasons[2] = "Autumn";
    println!("{}", seasons[2]);

    for season in seasons.iter() {
        println!("Season: {}", season);
    }

    seasons.iter().for_each(|season| {
        println!("Season via closure: {}", season);
    });

    for i in 0..seasons.len() {
        println!("Season via index {}: {}", i, seasons[i]);
    }
}
