use clap::Parser;
use clap_stdin::FileOrStdin;
use std::{any::type_name, env};

#[derive(Parser)]
struct Cli {
    // string: MaybeStdin<String>,
    input: Vec<FileOrStdin>,
}
fn main() {
    let cli = Cli::parse();

    // match cli.string[..] {
    //     _ => println!("{:?}", cli.string),
    //     // _ => println!("something else happened!"),
    // }

    if let Ok(text) = env::var("stdin") {
        println!("stdout {text}");
    }

    for each in cli.input {
        if each.type_name()
        match each {
            _ => println!("{:?}", each.contents()),
            // _ => println!("something else happened!"),
        }
    }
}
