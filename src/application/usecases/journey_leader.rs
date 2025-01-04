use std::sync::Arc;

use anyhow::Result;

use crate::domain::repositories::{
    journey_ledger::JourneyLedgerRepository, quest_viewing::QuestViewingRepository,
};

pub struct JourneyLeaderUseCase<T1, T2>
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    journey_leader_repository: Arc<T1>,
    quest_viewing_repository: Arc<T2>,
}

impl<T1, T2> JourneyLeaderUseCase<T1, T2>
where
    T1: JourneyLedgerRepository + Send + Sync,
    T2: QuestViewingRepository + Send + Sync,
{
    pub fn new(
        &self,
        journey_leader_repository: Arc<T1>,
        quest_viewing_repository: Arc<T2>,
    ) -> Self {
        Self {
            journey_leader_repository,
            quest_viewing_repository,
        }
    }

    pub async fn in_journey(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!()
    }

    pub async fn to_completed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!()
    }

    pub async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        unimplemented!()
    }
}
