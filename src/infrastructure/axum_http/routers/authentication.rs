use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Router};

use crate::{
    application::usecases::authentication::AuthenticationUseCase,
    domain::repositories::{
        adventurers::AdventurersRepository, guild_commanders::GuildCommandersRepository,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            adventurers::AdventurersPostgres, guild_commanders::GuildCommandersPostgres,
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurers_repository = AdventurersPostgres::new(Arc::clone(&db_pool));

    let guild_commanders_repository = GuildCommandersPostgres::new(Arc::clone(&db_pool));

    let authentication_use_case = AuthenticationUseCase::new(
        Arc::new(adventurers_repository),
        Arc::new(guild_commanders_repository),
    );

    Router::new()
        .route("adventurers/login", post(adventurers_login))
        .route("adventurers/refresh-token", post(adventurers_refresh_token))
        .route("guild_commanders/login", post(guild_commanders_login))
        .route(
            "guild_commanders/refresh-token",
            post(guild_commanders_refresh_token),
        )
        .with_state(Arc::new(authentication_use_case))
}

pub async fn adventurers_login<T1, T2>(
    State(authenticate_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn adventurers_refresh_token<T1, T2>(
    State(authenticate_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn guild_commanders_login<T1, T2>(
    State(authenticate_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn guild_commanders_refresh_token<T1, T2>(
    State(authenticate_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    unimplemented!()
}
