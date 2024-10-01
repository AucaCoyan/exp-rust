use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about) ]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Sync,
    Monitor,
    DisplayConfig,
    DisplaySyncStatus,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Sync => println!("output sync"),
        Command::Monitor => println!("output monitor"),
        Command::DisplayConfig => println!("output DisplayConfig"),
        Command::DisplaySyncStatus => println!("output Display sync status"),

    }
    println!("Hello, world!");
}
