pub struct Discount(DiscountType);


/// Type of discount you want to apply
#[derive(Debug)]
pub enum DiscountType {
    Absolute(usize),
    Percentage(usize),
    TwoForOne,
    ThreeForTwo,
}

impl Discount {
    pub fn apply(&self, product: &mut Product) -> &mut Product {
        product.price = match self.0 {
            DiscountType::Absolute(number) => product.price - number,
            DiscountType::Percentage(percentage) => product.price * percentage,
            DiscountType::TwoForOne => 1,
            DiscountType::ThreeForTwo => 0,

        };
        return product
    }
}

pub struct Product<'a> {
    pub price: usize,
    pub name: &'a str,
}

fn have_fun() {
    let mut product1: Product = Product { price: 10, name: "scooter"};
    let discount1 : Discount = Discount(DiscountType::Percentage(25));
    discount1.apply(&mut product1);
}


