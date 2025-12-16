use variables::EXTERNAL_CONSTANT;

const TAX_RATE: f64 = 7.25;

fn main() {
    let income: i32 = 100000;
    println!("My income is {income} and my tax rate is {TAX_RATE}");
    println!("External constant from lib.rs?: {EXTERNAL_CONSTANT}");
}
