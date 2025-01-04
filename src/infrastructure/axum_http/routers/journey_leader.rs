use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::patch,
    Extension, Router,
};

use crate::{
    application::usecases::journey_leader::JourneyLeaderUseCase,
    domain::repositories::{
        journey_ledger::JourneyLedgerRepository, quest_viewing::QuestViewingRepository,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            journey_ledger::JourneyLedgerPostgres, quest_viewing::QuestViewingPostgres,
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let journey_leader_repository = JourneyLedgerPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));

    let journey_leader_use_case = JourneyLeaderUseCase::new(
        Arc::new(journey_leader_repository),
        Arc::new(quest_viewing_repository),
    );

    Router::new()
        .route("/in-journey/:quest_id", patch(in_journey))
        .route("/to-completed/:quest_id", patch(to_completed))
        .route("/to-failed/:quest_id", patch(to_failed))
        .with_state(Arc::new(journey_leader_use_case))
}

pub async fn in_journey<T1, T2>(
    State(journey_leader_use_case): State<Arc<JourneyLeaderUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn to_completed<T1, T2>(
    State(journey_leader_use_case): State<Arc<JourneyLeaderUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn to_failed<T1, T2>(
    State(journey_leader_use_case): State<Arc<JourneyLeaderUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
