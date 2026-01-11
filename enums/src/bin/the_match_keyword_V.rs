use std::fmt;
use std::fmt::Display;
use std::ops::Add;

enum Milk {
    Lowfat(i32),
    Whole,
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(2) => {
                println!("Delicious, 2% is my favorite!");
            }
            Milk::Lowfat(percent) => {
                println!("You've got the lowfat {percent} percent version!");
            }
            Milk::Whole => {
                println!("You've got the whole milk!");
            }
        }
    }

    fn print(&self) {
        match self {
            Milk::Lowfat(percent) => {
                println!("Milk type: Lowfat, Percent: {percent}");
            }
            Milk::Whole => {
                println!("Milk type: Whole");
            }
        }
    }

}

impl Display for Milk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Milk::Lowfat(percent) => write!(f, "{percent}"),
            Milk::Whole => write!(f, "Whole"),
        }
    }
}

impl Add<i32> for &Milk {
    type Output = i32;

    fn add(self, rhs: i32) -> Self::Output {
        match self {
            Milk::Lowfat(percent) => percent + rhs,
            Milk::Whole => rhs,
        }
    }
}

fn main() {
    Milk::Lowfat(2).drink();
    Milk::Lowfat(1).drink();
    Milk::Whole.drink();

    Milk::Lowfat(2).print();
    Milk::Whole.print();

    let milk: Milk = Milk::Lowfat(5);

    println!("Counting with enum value {} + 5 = {}", &milk, &milk + 5);
}
