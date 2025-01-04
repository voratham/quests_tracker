use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::guild_commanders::GuildCommandersRepository,
    value_objects::guild_commander_model::RegisterAdventurerModel,
};

pub struct GuildCommandersUseCase<T>
where
    T: GuildCommandersRepository + Send + Sync,
{
    guild_commanders_repository: Arc<T>,
}

impl<T> GuildCommandersUseCase<T>
where
    T: GuildCommandersRepository + Send + Sync,
{
    pub fn new(&self, guild_commanders_repository: Arc<T>) -> Self {
        Self {
            guild_commanders_repository,
        }
    }

    pub async fn register(
        &self,
        register_guild_commander_model: RegisterAdventurerModel,
    ) -> Result<i32> {
        unimplemented!()
    }
}
