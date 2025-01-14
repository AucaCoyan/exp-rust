use clap::{Args, Parser, Subcommand};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "espanso")]
#[command(about = "A Privacy-first, Cross-platform Text Expander")]
#[command(version = VERSION)]
#[command(long_about=None)]
#[command(arg_required_else_help = true)]
pub struct Arguments {
  #[command(subcommand)]
  pub command: Command,

  // Sets the level of verbosity
  #[arg(short, long, action = clap::ArgAction::Count)]
  pub verbose: u8,
}

#[derive(Subcommand, Debug)]
pub enum Command {
  /// Send a command to the espanso daemon
  #[clap(subcommand)]
  Cmd(CmdCommand),
  /// Shortcut to open the default text editor to edit config files
  Edit {
    /// Defaults to "match/base.yml". It contains the relative path of the file you
    /// want to edit, such as 'config/default.yml' or 'match/base.yml'.
    /// For convenience, you can also specify the name directly and espanso will
    /// figure out the path. For example, specifying 'email' is equivalent to 'match/email.yml'.
    target_file: Option<String>,
  },
  /// Add or remove the 'espanso' command from the PATH
  EnvPath(EnvPathArgs),
  /// Install a package
  Install { package_name: String },
  /// Print the daemon logs
  Log,
  /// List and execute matches from the CLI
  Match,
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
  /// Open the Espanso's search bar
  Search,
  /// Enable/Disable expansions
  Toggle,
}

#[cfg(target_os = "macos")]
#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
#[command(arg_required_else_help = true)]
pub struct EnvPathArgs {
  #[command(subcommand)]
  command: Option<EnvPathSubCommand>,

  #[arg(short, long)]
  prompt: bool,
}

#[cfg(not(target_os = "macos"))]
#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
#[command(arg_required_else_help = true)]
pub struct EnvPathArgs {
  #[command(subcommand)]
  command: Option<EnvPathSubCommand>,
}

#[derive(Debug, Subcommand)]
pub enum EnvPathSubCommand {
  /// Add 'espanso' command to PATH
  Register,
  /// Remove 'espanso' command from PATH
  Unregister,
}

#[cfg(target_os = "macos")]
#[derive(Subcommand, Debug)]
pub enum EnvPathArgs {
  /// Add 'espanso' command to PATH
  Register,
  /// Remove 'espanso' command from PATH
  Unregister,
}

