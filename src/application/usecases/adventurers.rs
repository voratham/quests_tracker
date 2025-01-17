use std::sync::Arc;

use anyhow::{Ok, Result};

use crate::{
    domain::{
        repositories::adventurers::AdventurersRepository,
        value_objects::adventurer_model::RegisterAdventurerModel,
    },
    infrastructure::argon2_hashing,
};

// NOTE - we cannot pass directly on trait but we can solve dynamic dispatch - Box<dyn AdventurersRepository> or dyn AdventurersRepository but it will runtime allocate resource, if you want to performance static we will using Generic

// Example
// pub struct AdventurersUseCase {
//     pub adventurers_repository: Arc<dyn AdventurersRepository>,
// }

// Handler - Axum
// UseCase
// Repository

pub struct AdventurersUseCase<T>
where
    // Send -> ส่งของข้าม thread , ownership ข้าม thread
    // Sync -> สามารถทำงานพร้อมกันหลาย ๆ Thread ได้
    T: AdventurersRepository + Send + Sync,
{
    adventurers_repository: Arc<T>,
}

impl<T> AdventurersUseCase<T>
where
    T: AdventurersRepository + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T>) -> Self {
        Self {
            adventurers_repository,
        }
    }

    pub async fn register(
        &self,
        mut register_adventurer_model: RegisterAdventurerModel,
    ) -> Result<i32> {
        let hashed_password = argon2_hashing::hash(register_adventurer_model.password.clone())?;

        let password_new = hashed_password;
        register_adventurer_model.password = password_new;

        let register_entity = register_adventurer_model.to_entity();

        let adventurer_id = self
            .adventurers_repository
            .register(register_entity)
            .await?;

        Ok(adventurer_id)
    }
}
