fn main() {
    println!("Coke == Pepsi: {}", "Coke" == "Pepsi");
    println!("Coke != Pepsi: {}", "Coke" != "Pepsi");
    println!("Coke == coke: {}", "Coke" == "coke");
    println!("Coke == \'Coke \': {}", "Coke" == "Coke ");
    println!("Coke == Coke: {}", "Coke" == "Coke");

    println!("13 == 13: {}", 13 == 13);
    println!("13 != 13: {}", 13 != 13);

    println!("26.1 == 26.1: {}", 26.1 == 26.1);
    println!("26.1 == 26.14: {}", 26.1 == 26.14);

    println!("13 == 13.1 as i32: {}", 13 == 13.1 as i32);

    println!("true == true: {}", true == true);
    println!("false == false: {}", false == false);
    println!("true != false: {}", true != false);
}
