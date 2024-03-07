use clap::{Parser, Subcommand};
use log::{debug, info, trace, warn, LevelFilter};
use simple_logger::SimpleLogger;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[clap(short, long)]
        list: bool,
    },

    #[arg(try_get_matches_from("help"))]
    Help,
}

fn main() {
    let cli_instance = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli_instance.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli_instance.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match cli_instance.verbose {
        0 => SimpleLogger::new()
            .with_level(LevelFilter::Error)
            .init()
            .unwrap(),
        1 => SimpleLogger::new()
            .with_level(LevelFilter::Info)
            .init()
            .unwrap(),
        // Trace mode is only available in debug mode for security reasons
        #[cfg(debug_assertions)]
        3 => SimpleLogger::new()
            .with_level(LevelFilter::Trace)
            .init()
            .unwrap(),
        _ => SimpleLogger::new()
            .with_level(LevelFilter::Debug)
            .init()
            .unwrap(),
    };

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    // error!("some error log");

    if let Some(handler) = &cli_instance.command {
        Errork
    } else {
        cli_instance.print_help()
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    match &cli_instance.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    // Continued program logic goes here...
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
