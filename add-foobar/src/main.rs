use chrono::prelude::*;
use std::env;
use std::process;

fn main() {
    // print the datetime
    let local: DateTime<Local> = Local::now();
    let formatted = format!("{}", local.format("%H:%M %d/%m/%Y"));

    println!("Starting program at {formatted}");

    // print the name of the crate
    // total of arguments
    // and PID of the process
    let name_of_app = env!("CARGO_PKG_NAME");
    let args_count: usize = env::args().count();
    let pid = process::id();

    println!("Running the program {name_of_app}, with {args_count} arguments with PID {pid}");

    // recieve the arguments
    let args: Vec<String> = env::args().collect();

    // run the for loop for every argument passed
    for (item, arg) in args.iter().enumerate() {
        println!("The argument is {item} and the arg is: '{arg}'");
    }
}
