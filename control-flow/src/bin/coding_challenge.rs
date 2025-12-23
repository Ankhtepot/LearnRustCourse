
fn main() {
    // Define a `color_to_number` function that accepts a 'color'
    // parameter (a string). Use if, else if, and else
    // statements to return a corresponding numeric value based
    // on the following rules:
    // 1. If the color is "red", return 1.
    // 2. If the color is "green", return 2.
    // 3. If the color is "blue", return 3.
    // 4. If the color is any other string, return 0.
    fn color_to_number(color: &str) -> i32 {
        return if color == "red" { 1 }
        else if color == "green" { 2 }
        else if color == "blue" { 3 }
        else { 0 };
    }

    println!("If-else loop: \"green\"");
    println!("{}", color_to_number("green"));

    // Refactor the function above to use the `match` statement
    // instead of if, else if, and else.
    fn color_to_number_match(color: &str) -> i32 {
        match color {
            "red" => 1,
            "green" => 2,
            "blue" => 3,
            _ => 0
        }
    }

    println!("Match: \"blue\"");
    println!("{}", color_to_number_match("blue"));
    // Define a `factorial` function that calculates the
    // factorial of a number. The factorial is the product
    // of multiplying a number by every incremental
    // number leading up to it, starting from 1.
    //
    // Examples:
    // The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
    // factorial(5) should return 120.
    //
    // The factorial of 4 is 4 * 3 * 2 * 1 = 24
    // factorial(4) should return 24.

    // Implement two solutions/functions for the problem.
    // The first solution should not use recursion.
    // The second solution should use recursion.

    fn factorial(mut num: i32) -> i32 {
        let mut result = num;
        while num > 1 {
            result *= num - 1;
            num -= 1;
        }

        result
    }

    let number = 4;
    println!("Factorial of: \"{number}\"");
    println!("{}", factorial(number));


    fn factorial_recursion(mut num: i32, mut result_value: i32) -> i32 {
        if num > 1 {
            result_value *= num - 1;
            result_value = factorial_recursion(num - 1, result_value);
        }
        result_value
    }

    fn factorial_recursion_correct(num: i32) -> i32 {
        if num > 1 {
            num * factorial_recursion_correct(num - 1)
        } else {
            1  // Base case
        }
    }

    let number = 4;
    // let recursive_result = number;
    println!("Factorial with recursion of: \"{number}\"");
    println!("{}", factorial_recursion_correct(number));
}
