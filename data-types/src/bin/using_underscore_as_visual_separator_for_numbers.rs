#![allow(unused_variables)]

use nameof::name_of;
fn main() {
    // Using underscores as visual separators in numeric literals, they have no effect on the actual value.
    let sixteen_bit_signed: i16 = 32_500;
    let sixteen_bit_signed: i16 = -3_2_500;
    let sixteen_bit_signed: i16 = -3_2_5_00;

    println!("Using dbg! macro:");
    dbg!(sixteen_bit_signed);
    println!("Using nameof crate:");
    println!("{}: {}", name_of!(sixteen_bit_signed), sixteen_bit_signed);
    println!("Using nameof without import (nameof:name_of!(...)):");
    println!("{}: {}", nameof::name_of!(sixteen_bit_signed), sixteen_bit_signed);
}
