use std::process::exit;

use clap::{Parser, Subcommand};
use log::{debug, error, info};

use crate::config::AccountType;

mod config;
mod sync_engine;

const APP_CONFIG: config::App = config::App {
    dry_run: false,
    account_type: AccountType::Personal,
    connection_timeout: 100,
    data_timeout: 100,
    dns_timeout: 100,
    ip_protocol_version: 100,
    operation_timeout: 100,

};
const HTTP_URL: &str = "https://login.microsoftonline.com";

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand, Debug)]
enum Command {
    Sync,
    Monitor,
    DisplayConfig,
    DisplaySyncStatus,
}

fn main() {
    env_logger::init();

    info!("Application started");
    debug!("Application version __TODO__");
    let args = Cli::parse();

    let online = test_internet_reachability(&APP_CONFIG);
    if !online {
        error!("Whoops, not connected online");
        exit(1);
    }

    if APP_CONFIG.dry_run || has_no_sync_operation_been_requestes(&APP_CONFIG) {
        todo!()
    }

    debug!("Passed in args: {:?}", args);
    match args.command {
        Command::Sync => sync(&APP_CONFIG),
        Command::Monitor => println!("output monitor"),
        Command::DisplayConfig => println!("output DisplayConfig"),
        Command::DisplaySyncStatus => println!("output Display sync status"),
    }
    println!("Hello, world!");
}

fn sync(app_config: &config::App) {
    info!("running Sync");
    sync_engine::SyncEngine::sync_one_drive_account_to_local_disk();

    match app_config.account_type {
        AccountType::Personal => println!("we have a personal account!"),
        AccountType::Business => todo!(),
    }
}

fn test_internet_reachability(app_config: &config::App) -> bool {
    info!("Attempting to contact Microsoft OneDrive Login Service __todo__ ");
    
    let _dns_timeout = app_config.dns_timeout;
    let _connection_timeout = app_config.connection_timeout;
    let _data_timeout = app_config.data_timeout;
    let _operation_timeout = app_config.operation_timeout;
    let _protocol_version= app_config.ip_protocol_version;

    true
}

fn has_no_sync_operation_been_requestes(app_config: &config::App)-> bool {
    if app_config.dry_run 
    // and all the other cases where a db should be copied instead of written
    {
        return  true
    }
    false
}