use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::quest_viewing::QuestViewingRepository,
    value_objects::{board_checking_filter::BoardCheckingFilter, quest_model::QuestModel},
};

pub struct QuestViewingUseCase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    quest_viewing_repository: Arc<T>,
}

impl<T> QuestViewingUseCase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    pub fn new(&self, quest_viewing_repository: Arc<T>) -> Self {
        Self {
            quest_viewing_repository,
        }
    }

    pub async fn view_details(&self, quest_id: i32) -> Result<QuestModel> {
        unimplemented!()
    }

    pub async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestModel>> {
        unimplemented!()
    }
}
