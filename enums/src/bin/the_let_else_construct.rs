#![allow(unused)]

#[derive(Debug)]
enum Milk {
    Whole,
    Lowfat(i32),
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::NonDairy {
        kind: String::from("Oat"),
    };

    let Milk::NonDairy { kind } = my_beverage else {
        println!("You do not have the nondairy milk");
        return; // <- needs to be there to end the else block for case when
        // the variable is not a NonDairy
    };

    println!("{kind} milk is available here");

    let low_fat = Milk::Lowfat(2);

    if let Milk::Lowfat(amount) = low_fat {
        println!("Low-fat milk with {amount} grams of fat");
    } else {
        println!("Not low-fat milk");
    }

    println!("{low_fat:#?}");
    println!("{low_fat:#?}");
}
