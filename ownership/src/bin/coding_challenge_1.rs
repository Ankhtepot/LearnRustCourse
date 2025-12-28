fn main() {
    // Declare a `is_concert` variable set to a boolean.
    // Declare a `is_event` variable assigned to `is_concert`.
    // Will Rust move ownership? State your answer, then confirm
    // by trying to printing both variables out.
    let is_concert: bool = true;
    let is_event: bool = is_concert;
    println!("is_concert == {}", is_concert);
    println!("is_event == {}", is_event);

    // Declare a `sushi` variable to set to a string literal of "Salmon"
    // Declare a `dinner` variable assigned to the `sushi` variable.
    // Will Rust move ownership? State your answer, then confirm
    // by trying to print both variables out.
    let sushi: &str = "Salmon";
    let dinner: &str = sushi;
    println!("sushi = {}", sushi); // can - pointer points to same space in a memory
    println!("dinner = {}", dinner);

    // Repeat the previous example but use a heap String instead.
    // Will Rust move ownership? Explain why the result is different
    // from the previous operation.
    let sushi: String = String::from("Salmon");
    let dinner: String = sushi;
    // println!("sushi = {}", sushi); // can't - the ownership was moved
    println!("dinner = {}", dinner);

    // The `clear` method modifies a heap String to have no content.
    // Declare an `eat_meal` function that accepts a `meal` parameter
    // of type String. In the body of `eat_meal`, invoke the `clear`
    // method on the `meal` parameter.

    eat_meal(String::from("Some meal"));
    fn eat_meal(mut meal: String) {
        meal.clear();
        println!();
        println!("meal after clean = {}", meal);
    }

    // In the `main` function, invoke the `eat_meal` function and pass
    // in your "Salmon" String. Explain what happens when the eat_meal
    // function runs. Describe the complete movement of ownership of
    // the "Salmon" String throughout the program.

    println!("Eating dinner");
    let eaten_meal =eat_meal_return(dinner);
    println!("Eating dinner");
    println!("The rest: {:?}", eaten_meal);

    // Say we want to keep the String around after `eat_meal` is
    // called. How can we continue to have access to the String in
    // the `main` function? Print out the (empty) String.

    fn eat_meal_return(mut meal: String) -> String {
        meal.clear();
        println!();
        meal
    }
}
