fn main() {

    // Declare an i32 variable assigned to 1337.
    // Use the underscore character to add a visual
    // separator between the numbers.
    let integer: i32 = 13_37;

    // Cast the i32 to an i16 integer and assign the result
    // to a separate variable.
    let integer_i16: i16 = integer as i16;

    // Declare a floating-point value of your choosing.
    // Print out the number with 3 digits of precision.
    let float_number: f64 = 3.141592;
    println!("{float_number:.3}");

    // Declare a 'with_milk' variable set to a Boolean.
    // Declare a 'with_sugar' variable set to a Boolean.
    let with_milk: bool = true;
    let with_sugar: bool = true;

    // Declare an 'is_my_type_of_coffee' variable. It should
    // be set to true if the coffee has both milk and sugar.
    let is_my_type_of_coffee: bool = with_milk && with_sugar;

    // Declare an `is_acceptable_coffee` variable. It should
    // be set to true if the coffee has either milk or
    // sugar.
    let is_acceptable_coffee: bool = with_milk || with_sugar;

    // Declare an array with four i8 integers of your choosing
    // Print out the array in its Debug representation.
    let array_4 = [5,6,7,8];
    dbg!(array_4);
    println!("{array_4:#?}");

    // Declare a tuple consisting of the integer, float,
    // a Boolean, and the array that you previously declared.
    // Print out the tuple in its Debug representation.

    let tuple = (16, 5.678, true, array_4);
    dbg!(tuple);
    println!("{tuple:#?}");
}
