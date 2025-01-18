use std::{sync::Arc, time::Duration};

use anyhow::{Ok, Result};
use axum::{http::Method, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;

use crate::{
    config::config_model::DotEnvyConfig,
    infrastructure::{axum_http::routers, postgres::postgres_connection::PgPoolSquad},
};

use super::default_routers::{health_check, not_found};

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    let app = Router::new()
        .fallback(not_found)
        .route("/healthcheck", get(health_check))
        .nest(
            "/quest-ops",
            routers::quest_ops::routes(Arc::clone(&&db_pool)),
        )
        .nest(
            "/quest-viewing",
            routers::quest_viewing::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/journey-ledger",
            routers::journey_ledger::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/guild-commanders",
            routers::guild_commanders::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/crew-switchboard",
            routers::crew_switchboard::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/authentication",
            routers::authentication::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/adventurers",
            routers::adventurers::routes(Arc::clone(&db_pool)),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(
            config.server.timeout, //  middleware 4
        )))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?,
        )) //  middleware 3
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any), //  middleware 2 // TODO: not use on production
        )
        .layer(TraceLayer::new_for_http()); //  middleware 1

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let listener = TcpListener::bind(addr).await?;

    info!("Server is running on port {}", config.server.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("🔴 Failed to install CTRL+C signal handler");
    };

    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("🔴 Failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {
            info!("🔴 Received Ctrl-C, shutting down");
        },
        _ = terminate => {
            info!("🔴 Received terminate signal, shutting down");
        }
    }
}
