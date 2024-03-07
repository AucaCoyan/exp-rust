use clap::Parser;
use log::trace;
use serde_json::Result;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[arg(help = "the configuration file")]
    config_file: PathBuf,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    trace!("recieved config_file: {:?}", cli.config_file);
    clone_repos(&cli.config_file);
}

fn clone_repos(config: &PathBuf) -> Result<()> {
    if !config.exists() {
    } else {
        let config = serde_json::from_str(config)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn clap_cli_contruction() {
        Cli::command().debug_assert();
    }
}
