#![allow(unused)]

#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
                println!("Your item is being prepared")
            }
            OnlineOrderStatus::Delivered => {
                println!("Your item has arrived")
            }
            other_status => {
                println!("Your item is {other_status:?}")
            }
        }
    }
}

fn main() {
    OnlineOrderStatus::Ordered.check();
    OnlineOrderStatus::Packed.check();
    OnlineOrderStatus::Shipped.check();
    OnlineOrderStatus::Delivered.check();
}
