use std::{io::stdin, num::IntErrorKind};
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
    println!("Box simulator 1.0");

    let mut response = String::new();

    println!("Write the box number: \n(between 0 and 65 535)");

    stdin().read_line(&mut response).expect("Failed input");

    let tiny_box = match response
        .trim_end() // remove the \r\n
        .parse::<u16>()
    {
        Ok(number) => number,
        Err(error) => match error.kind() {
            IntErrorKind::PosOverflow => panic!("You can't pick a number greater than 65 535"),
            IntErrorKind::NegOverflow => panic!("You can't pick a number less than 0"),
            IntErrorKind::Empty => panic!("I read an empty value"),
            other_error => {
                panic!("Something wrong has happened: {:?}", other_error)
            }
        },
    };

    println!("you chose {:?}", tiny_box)
}

/*

    let exit = false;

    while exit == false {

        let res = match buffer.trim_end() {
            "q" => println!("you pressed q"),
            _ => println!("something has gone wrong"),
        };

        println!("{:?}", res)
    }
}
*/