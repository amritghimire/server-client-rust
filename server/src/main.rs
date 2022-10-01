use clap::Parser;

use server::configuration::Settings;
use server::{startup, Opt};

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    let settings = Settings::new().expect("Failed to read configuration file");

    startup::start(opt, settings).await;
}
