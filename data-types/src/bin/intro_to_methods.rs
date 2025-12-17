
fn main() {
    let val: i32 = -15;
    println!("{}", val.abs());
    println!("{}", absolute(val));

    let empty_space = "     my content    ";
    println!("{}", empty_space.trim());

    println!("{}", val.pow(2));
    println!("Extension trait method:");
    println!("{}", val.pow2());

    println!("{}", val.pow(3));
}

fn absolute(value: i32) -> i32 {
    if value < 0 {
        -value
    } else {
        value
    }
}

pub trait Pow {
    fn pow2(self) -> Self;
}

impl Pow for i32 {
    fn pow2(self) -> Self {
        let result = self * self;

        result
    }
}
