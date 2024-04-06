use clap::Parser;
use gcloud_sdk::{GoogleApi, GoogleApiClient};

use google_calendar;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    people: Vec<String>
}

#[tokio::main]
async fn main() {
    let api_key = dotenv::var("API_KEY");
    let client_id = dotenv::var("CLIENT_ID");
    let client = google_calendar::Client();

    let cli = Args::parse();

    for person in cli.people {
        println!("You invited {person}!");
    }
}
