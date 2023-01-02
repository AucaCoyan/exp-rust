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
    let args = "0";
    let pid = process::id();

    println!("Running the program {name_of_app}, with {args} arguments with PID {pid}");
    print_arguments()
}

fn print_arguments() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
