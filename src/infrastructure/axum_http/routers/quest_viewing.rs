use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::{
    application::usecases::quest_viewing::QuestViewingUseCase,
    domain::{
        repositories::quest_viewing::QuestViewingRepository,
        value_objects::board_checking_filter::BoardCheckingFilter,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, repositories::quest_viewing::QuestViewingPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let quest_viewing_repository = QuestViewingPostgres::new(db_pool);

    let quest_viewing_use_case = QuestViewingUseCase::new(Arc::new(quest_viewing_repository));

    Router::new()
        .route("/:quest_id", get(view_details))
        .route("/board_checking", get(board_checking))
        .with_state(Arc::new(quest_viewing_use_case))
}

pub async fn view_details<T>(
    State(quest_viewing_use_case): State<Arc<QuestViewingUseCase<T>>>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn board_checking<T>(
    State(quest_viewing_use_case): State<Arc<QuestViewingUseCase<T>>>,
    filter: Query<BoardCheckingFilter>,
) -> impl IntoResponse
where
    T: QuestViewingRepository + Send + Sync,
{
    unimplemented!()
}
