#![allow(unused_variables)]

fn main() {
    let miles_away = 50;
    let miles_away_i8 = miles_away as i8;
    let miles_away_u8: u8 = miles_away as u8;

    let miles_away: f64 = 100.329032;
    let miles_away_f32 = miles_away as f32;
    let miles_away_int = miles_away as i32;
    // direct use of nameof crate without import
    println!("{}: {miles_away_int}", nameof::name_of!(miles_away_int));
}
