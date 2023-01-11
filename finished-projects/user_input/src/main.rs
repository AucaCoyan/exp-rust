use std::io::stdin;

fn main() {
    let mut response = String::new();

    println!("type sth: ");

    stdin().read_line(&mut response).expect("Failed input");

    println!("you wrote {}", response)
}
