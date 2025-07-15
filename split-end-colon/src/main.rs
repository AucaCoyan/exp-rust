fn main() {
    let vector = String::from("one;two;tree;;");
    let result: Vec<&str> = vector.split(';').collect();

    println!("result:\n{result:#?}");
}
