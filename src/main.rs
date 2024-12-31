use std::process;

use quests_tracker::config::config_loader;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    //  casting log level to debug
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(error) => {
            error!("Failed to load ENV: {}", error);
            process::exit(1);
        }
    };

    info!("ENV has been loaded");
}
