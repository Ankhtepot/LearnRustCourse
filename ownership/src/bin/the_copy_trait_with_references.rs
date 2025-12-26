fn main() {
    let ice_cream = "Cookies and Cream";
    let dessert = ice_cream;
    // Notice no movement of ownership - &str is special in this
    println!("{ice_cream} {dessert}.");
}
