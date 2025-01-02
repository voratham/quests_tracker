use std::{process, sync::Arc};

use quests_tracker::{
    config::config_loader,
    infrastructure::{axum_http::http_serve::start, postgres::postgres_connection},
};
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
            error!("🔴 Failed to load ENV: {}", error);
            process::exit(1);
        }
    };

    info!("ENV has been loaded");

    let postgres_pool = match postgres_connection::establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("🔴 Failed to establish connection to postgres {}", e);
            process::exit(1);
        }
    };

    info!("Postgres connection has been established");

    start(Arc::new(dotenvy_env), Arc::new(postgres_pool))
        .await
        .expect("🔴 Failed to start server")
}
