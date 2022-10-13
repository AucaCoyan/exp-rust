use std::io::stdin;

enum Response {
    Quit,
    Edit,
    Print,
}

fn main() {
    println!("Welcome to");

    let mut exit = false;
    let mut input = String::new();

    while exit == false {
        println!();
        println!("Box simulator 1.0");
        match stdin().read_line(&mut input) {
            Ok(n) => {
                let select = n;
            }
            Err(e) => {
                println!("something has gone wrong")
            }
        }
        print!("{}", &n)
    }

    exit = false
}
