use std::io::stdin;
/*
struct Box {
    contents: String,
    id: u32,
}

enum Response {
    Quit,
    Edit,
    Print,
}
*/
fn main() {
    println!("Welcome to");

    let exit = false;
    let mut buffer = String::new();

    while exit == false {
        println!();
        println!("Box simulator 1.0");
        stdin().read_line(&mut buffer);
        let res = match buffer.trim_end() {
            "q" => println!("you pressed q"),
            _ => println!("something has gone wrong"),
        };

        println!("{:?}", res)
    }
}
