use clap::{App, Arg, SubCommand};

#[derive(Debug)]
enum Action {
    Add,
    Sub,
    List(ListOptions),
}

#[derive(Debug)]
struct ListOptions {
    all: bool,
}

fn main() {
    let matches = App::new("my-app")
        .version("1.0")
        .author("Your Name <your_email@example.com>")
        .about("A simple CLI app for performing calculations and listing items")
        .setting(clap::AppSettings::DeriveSubcommands)
        .subcommand(
            SubCommand::with_name("calc")
                .about("Perform basic calculations")
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Adds two numbers")
                        .arg(Arg::with_name("num1").index(1).required(true).takes_value(true))
                        .arg(Arg::with_name("num2").index(2).required(true).takes_value(true)),
                )
                .subcommand(
                    SubCommand::with_name("sub")
                        .about("Subtracts two numbers")
                        .arg(Arg::with_name("num1").index(1).required(true).takes_value(true))
                        .arg(Arg::with_name("num2").index(2).required(true).takes_value(true)),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists items")
                .arg(Arg::with_name("all").short("a").long("all").help("List all items")),
        )
        .get_matches();

    let action = match matches.subcommand() {
        Some(matches) => match matches.name() {
            "calc" => {
                match matches.subcommand() {
                    Some(matches) => match matches.name() {
                        "add" => Action::Add,
                        "sub" => Action::Sub,
                        _ => unreachable!(),
                    },
                    None => unreachable!(),
                }
            }
            "list" => {
                let all = matches.is_present("all");
                Action::List(ListOptions { all })
            }
            _ => unreachable!(),
        },
        None => unreachable!(),
    };

    // Perform chosen action
    match action {
        Action::Add { .. } => {
            // Extract and parse numbers
            let num1 = matches.value_of("num1").unwrap().parse::<i32>().unwrap();
            let num2 = matches.value_of("num2").unwrap().parse::<i32>().unwrap();

            println!("Result: {}", num1 + num2);
        }
        Action::Sub { .. } => {
            // Extract and parse numbers
            let num1 = matches.value_of("num1").unwrap().parse::<i32>().unwrap();
            let num2 = matches.value_of("num2").unwrap().parse::<i32>().unwrap();

            println!("Result: {}", num1 - num2);
        }
        Action::List(ListOptions { all }) => {
            // Simulate listing items
            println!("Listing items...");
            if all {
                println!("  (all items)");
            } else {
                println!("  (limited items)");
            }
        }
    }
}
