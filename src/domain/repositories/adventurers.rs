use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::{
    entities::adventurers::AdventurerEntity,
    value_objects::adventurer_model::RegisterAdventurerModel,
};

#[async_trait]
#[automock] // mock generate
pub trait AdventurersRepository {
    async fn register(&self, register_adventurer_model: RegisterAdventurerModel) -> Result<i32>;

    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity>;
}
