pub fn remove_char(s: &str) -> String {
    let mut full_string = String::from(s);
    full_string.pop();
    full_string.remove(0);
    return full_string;
}

fn main() {
    println!("{}", remove_char("eloquent"))
}
