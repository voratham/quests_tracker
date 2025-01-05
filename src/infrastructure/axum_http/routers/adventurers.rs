use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::{
    application::usecases::adventurers::AdventurersUseCase,
    domain::{
        repositories::adventurers::AdventurersRepository,
        value_objects::adventurer_model::RegisterAdventurerModel,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::adventurers::AdventurersPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    // init layer
    // repository
    // usecase
    // router state

    let adventurers_repository = AdventurersPostgres::new(db_pool);

    let adventurers_use_case = AdventurersUseCase::new(Arc::new(adventurers_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(adventurers_use_case))
}

pub async fn register<T>(
    State(adventurers_use_case): State<Arc<AdventurersUseCase<T>>>,
    Json(register_adventurer_model): Json<RegisterAdventurerModel>,
) -> impl IntoResponse
where
    T: AdventurersRepository + Send + Sync,
{
    match adventurers_use_case
        .register(register_adventurer_model)
        .await
    {
        Ok(adventurer_id) => (
            StatusCode::CREATED,
            format!("Register adventurer id: {} successfully", adventurer_id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
