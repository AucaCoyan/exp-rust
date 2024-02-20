use clap::{Parser, Subcommand};
use log::{debug, info, trace, warn, LevelFilter};
use simple_logger::SimpleLogger;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,

    #[clap(subcommand)]
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
    } else {
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
    fn clap_cli_construction() {
        app().debug_assert();
    }
}
