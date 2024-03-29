use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, setting = clap::AppSettings::DeriveSubcommands)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Calc(CalcCommands),
    List(ListOptions),
}

#[derive(Subcommand)]
enum CalcCommands {
    Add { num1: i32, num2: i32 },
    Sub { num1: i32, num2: i32 },
}

#[derive(Parser)]
struct ListOptions {
    #[clap(short, long)]
    all: bool,
}

fn main() {
    let Cli { command } = Cli::parse();

    match command {
        Commands::Calc(calc_command) => match calc_command {
            CalcCommands::Add { num1, num2 } => {
                println!("Result: {}", num1 + num2);
            }
            CalcCommands::Sub { num1, num2 } => {
                println!("Result: {}", num1 - num2);
            }
        },
        Commands::List(options) => {
            println!("Listing items...");
            if options.all {
                println!("  (all items)");
            } else {
                println!("  (limited items)");
            }
        }
    }
}
