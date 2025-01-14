use log::LevelFilter;

mod cli;

fn main() {
    let args = cli::Arguments::parse();

    let log_level = match args.verbose {
        0 | 1 => {
            // println!("Debug mode is off");
            LevelFilter::Info
        }
        2 => {
            println!("Debug mode is on");
            LevelFilter::Debug
        }
        // Trace mode is only available in debug mode for security reasons
        #[cfg(debug_assertions)]
        3 => LevelFilter::Trace,
        _ => LevelFilter::Warn,
    };

    match args.command {
        cli::Command::Cmd { .. } => println!("something anything"),
        cli::Command::Edit { target_file } => {
            if target_file.is_some() {
                println!("the file {:#?}", target_file)
            } else {
                println!("`espanso edit` (empty) was passed")
            }
        }
        cli::Command::EnvPath(..) => println!("some dummy output"),
        cli::Command::Install { .. } => println!("some dummy output"),
        cli::Command::Log {} => println!("some dummy output"),
        cli::Command::Match {} => println!("some dummy output"),
        cli::Command::Package {} => println!("some dummy output"),
        cli::Command::Path => println!("some dummy output"),
        cli::Command::Restart {} => println!("some dummy output"),
        cli::Command::Service(command) => println!("{command:?}"),
        cli::Command::Start {} => println!("some dummy output"),
        cli::Command::Status {} => println!("some dummy output"),
        cli::Command::Stop {} => println!("some dummy output"),
        cli::Command::Uninstall {} => println!("some dummy output"),
        cli::Command::Workaround {} => println!("some dummy output"),
    }

    println!("Hello, world!");
}
