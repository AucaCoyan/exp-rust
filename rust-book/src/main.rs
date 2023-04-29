#![allow(unused)]
fn main() {
    let may_have_value_1: Option<i32> = Some(10);
    let may_have_value_2: Option<i32> = Some(20);
    println!("{}", may_have_value_1 + may_have_value_2)
}
