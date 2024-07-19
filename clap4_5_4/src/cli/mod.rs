use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "espanso")]
#[command(about = "A Privacy-first, Cross-platform Text Expander")]
#[command(version = "2.2.1")]
#[command(long_about=None)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,
    //command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Send a command to the espanso daemon
    #[clap(subcommand)]
    Cmd(CmdCommand),
    /// Shortcut to open the default text editor to edit config files
    Edit {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// Add or remove the 'espanso' command from the PATH
    #[clap(subcommand)]
    EnvPath(EnvPathCommand),
    /// Prints this message or the help of the given subcommand(s)
    //Help,
    /// Install a package
    Install { package_name: String },
    /// Print the daemon logs
    Log,
    /// List and execute matches from the CLI
    Match,
    /// Automatically migrate legacy config files to the new v2 format
    Migrate,
    /// Package-management commands
    Package,
    /// Prints all the espanso directory paths to easily locate configuration and matches
    Path,
    /// Restart the espanso service
    Restart,
    /// A collection of commands to manage the Espanso service (for example, enabling auto-start on system boot).
    #[clap(subcommand)]
    Service(ServiceCmd),
    /// Start espanso as a service
    Start,
    /// Check if the espanso daemon is running or not
    Status,
    /// Stop espanso service
    Stop,
    /// Remove a package
    Uninstall,
    /// A collection of workarounds to solve some common problems
    Workaround,
}

#[derive(Subcommand, Debug)]
pub enum ServiceCmd {
    /// Check if espanso is registered as a system service
    Check,
    // Prints this message or the help of the given subcommand(s)
    // Help,
    /// Register espanso as a system service
    Register,
    ///Restart the espanso service
    Restart,
    /// Start espanso as a service
    Start,
    /// Check if the espanso daemon is running or not.
    Status,
    /// Stop espanso service
    Stop,
    /// Unregister espanso from system services
    Unregister,
}

#[derive(Subcommand, Debug)]
pub enum CmdCommand {
    /// Disable expansions
    Disable,
    /// Enable expansions
    Enable,
    // Print this message or the help of the given subcommand(s)
    // Help,
    /// Open the Espanso's search bar
    Search,
    /// Enable/Disable expansions
    Toggle,
}

#[derive(Subcommand, Debug)]
pub enum EnvPathCommand {
    /// Add 'espanso' command to PATH
    Register,
    /// Remove 'espanso' command from PATH
    Unregister,
    // Print this message or the help of the given subcommand(s)
    // Help,
}
