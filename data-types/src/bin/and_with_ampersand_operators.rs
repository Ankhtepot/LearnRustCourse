fn main() {
    let purchased_ticket = true;
    let plane_on_time = true;
    let making_event = purchased_ticket && plane_on_time;
    println!("It is {} that I will arrive as expected: (purchase: {purchased_ticket}, on time: {plane_on_time}).", making_event);
}
