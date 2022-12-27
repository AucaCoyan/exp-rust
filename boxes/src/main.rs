use std::{fmt::Error, io::stdin, num};
struct Box {
    contents: String,
    id: u16,
}

/*
enum Response {
    Quit,
    Edit,
    Print,
}
*/

fn match_ParseIntError(error: num::ParseIntError) {
    match error.kind() {
        num::IntErrorKind::PosOverflow => panic!("You can't pick a number greater than 65 535"),
        num::IntErrorKind::NegOverflow => panic!("You can't pick a number less than 0"),
        num::IntErrorKind::Empty => panic!("I read an empty value"),
        num::IntErrorKind::InvalidDigit => {
            println!("this is the error {}", error); // this line returns 'invalid digit found in string', it doesn't return the original response
            panic!("this is not a digit!")
        }
        other_error => {
            panic!("Something wrong has happened: {:?}", other_error)
        }
    }
}

fn pick_number() -> Result<u16, Error> {
    let mut response = String::new();

    println!("Write the box number: \n(between 0 and 65 535)");

    stdin().read_line(&mut response).expect("Cannot read line!");

    let tiny_box: Box = match response
        .trim_end() // remove the \r\n
        .parse::<u16>()
    {
        Ok(number) => Box {
            contents: String::new(),
            id: number,
        },
        Err(error) => match_ParseIntError(error),
    };
    Ok(tiny_box.id)
}

fn main() {
    println!("Welcome to");
    println!("Box simulator 1.0");

    loop {
        println!("{:?}", pick_number());
    }
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
