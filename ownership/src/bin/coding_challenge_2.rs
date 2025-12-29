fn main() {
    // Let's model a road trip!
    //
    // Define a `start_trip` function that creates and returns
    // a String of "The plan is..."

    fn start_trip() -> String {
        String::from("The plan is...")
    }

    // Invoke the `start_trip` function in `main` and save its
    // return value to a `trip` variable.

    let mut trip = start_trip();

    // We want to pass the String to three separate functions
    // that will mutate the String without transferring ownership.
    //
    // Define a `visit_philadelphia` function that concatenates
    // the text "Philadephia" to the end of the String. Invoke
    // the function in `main`. Then, invoke `push_str` on the String
    // to concatenate the content " and " to the end. Mak sure to
    // include the spaces.

    fn visit_philadelphia(trip:&mut String) {
        trip.push_str("Philadelphia");
    }

    visit_philadelphia(&mut trip);
    trip.push_str(" and ");

    // Define a `visit_new_york` function that concatenates the
    // text "New York" to the end of the String. Invoke the function
    // in `main`. Repeat the previous logic to concatenate " and "
    // to the end of the String.

    fn visit_new_york(trip: &mut String) {
        trip.push_str("New York");
    }

    visit_new_york(&mut trip);
    trip.push_str(" and ");

    // Define a `visit_boston` function that concatenates the
    // text "Boston." to the end of the String. Invoke the function
    // in `main`. Concatenate a period to the end of the
    // String/sentence.

    fn visit_boston(trip: &mut String) {
        trip.push_str("Boston");
    }

    visit_boston(&mut trip);

    // Define a `show_itinerary` function that will print out
    // the final version of the String. Find a way to do so
    // without transferring ownership.

    fn show_itinerary(trip: &String) {
        println!("{trip}");
    }

    // Invoke `show_itinerary`. The final output should be:

    show_itinerary(&trip);

    // "The plan is...Philadelphia and New York and Boston."
}
