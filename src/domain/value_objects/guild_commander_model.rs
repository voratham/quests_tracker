use serde::{Deserialize, Serialize};

use crate::domain::entities::guild_commanders::RegisterGuildCommanderEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterAdventurerModel {
    pub username: String,
    pub password: String,
}

impl RegisterAdventurerModel {
    pub fn to_entity(&self) -> RegisterGuildCommanderEntity {
        RegisterGuildCommanderEntity {
            username: self.username.clone(),
            password: self.password.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
