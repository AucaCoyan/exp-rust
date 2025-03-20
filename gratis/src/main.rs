use gratis::{DiscountType, Product};

fn main() {
    let bucket = vec![Product {
        price: 10,
        name: "shampoo",
    }];

    let _discount = DiscountType::Absolute(10);

    println!("{_discount:?}")
}
