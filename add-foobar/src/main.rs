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
        // run the command
        let output = process::Command::new("rg")
            .args(["foobar", arg.as_str()])
            .output()
            .expect("something has gone wrong");

        // I need to convert the output from ASCI extended to normal characters
        // so transform the type Vec<u8, Global> into &[u8]
        let output_transformed = output.stdout.as_slice();

        // read the bytes into a strig
        let result = String::from_utf8_lossy(output_transformed);

        // read the result
        println!("{result:?}")
    }
}
