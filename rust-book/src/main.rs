use std::str::FromStr;

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: Option<u64>,
}

fn main() {
    let new_string = String::from("Auca esta programando");
    let Auca = User {
        active: true,
        username: String::new(),
        email: String::new(),
        sign_in_count: None,
    };

    println!("{:?}", Auca.sign_in_count)
}
