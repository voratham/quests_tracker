use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, post, Router},
    Extension,
};

use crate::{
    application::usecases::crew_switchboard::CrewSwitchboardUseCase,
    domain::repositories::{
        crew_switchboard::CrewSwitchboardRepository, quest_viewing::QuestViewingRepository,
    },
    infrastructure::{
        axum_http::middlewares::adventurers_authorization,
        postgres::{
            postgres_connection::PgPoolSquad,
            repositories::{
                crew_switchboard::CrewSwitchBoardPostgres, quest_viewing::QuestViewingPostgres,
            },
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    // ถึงแม้ db_pool จะถูก clone มาจาก parameter แต่ข้างใน ของ  CrewSwitchBoardPostgres::new ยังมีการเรียก owner ซ้อนกัน ที่เราทำแบบนี้ เพื่อป้องกันความปลอดภัย
    let crew_switchboard_repository = CrewSwitchBoardPostgres::new(Arc::clone(&db_pool));

    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));

    let crew_switchboard_use_case = CrewSwitchboardUseCase::new(
        Arc::new(crew_switchboard_repository),
        Arc::new(quest_viewing_repository),
    );

    Router::new()
        .route("/join/:quest_id", post(join))
        .route("/leave/:quest_id", delete(leave))
        .route_layer(middleware::from_fn(adventurers_authorization))
        .with_state(Arc::new(crew_switchboard_use_case))
}

pub async fn join<T1, T2>(
    State(crew_switchboard_usecase): State<Arc<CrewSwitchboardUseCase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>, // in case if client access with token extension will extract auto (feature of axum framework)
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match crew_switchboard_usecase.join(quest_id, adventurer_id).await {
        Ok(()) => (
            StatusCode::OK,
            format!(
                "Adventurer id: {} joined quest id: {} successfully",
                adventurer_id, quest_id
            ),
        ),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub async fn leave<T1, T2>(
    State(crew_switchboard_usecase): State<Arc<CrewSwitchboardUseCase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewSwitchboardRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    match crew_switchboard_usecase
        .leave(quest_id, adventurer_id)
        .await
    {
        Ok(()) => (
            StatusCode::OK,
            format!(
                "Adventurer id: {} leaved quest id: {} successfully",
                adventurer_id, quest_id
            ),
        ),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}
