use clap::Parser;

mod cli;

fn main() {
    let args = cli::Arguments::parse();

    match args.command {
        cli::Command::Cmd { list } => println!("{list}"),
        cli::Command::Edit {} => println!(""),
        cli::Command::EnvPath {} => println!(""),
        cli::Command::Install {} => println!(""),
        cli::Command::Log {} => println!(""),
        cli::Command::Match {} => println!(""),
        cli::Command::Migrate {} => println!(""),
        cli::Command::Package {} => println!(""),
        cli::Command::Path => println!(""),
        cli::Command::Restart {} => println!(""),
        cli::Command::Service(command) => println!("{command:?}"),
        cli::Command::Start {} => println!(""),
        cli::Command::Status {} => println!(""),
        cli::Command::Stop {} => println!(""),
        cli::Command::Uninstall {} => println!(""),
        cli::Command::Workaround {} => println!(""),
    }
    // Continued program logic goes here...
    println!("Hello, world!");
}
