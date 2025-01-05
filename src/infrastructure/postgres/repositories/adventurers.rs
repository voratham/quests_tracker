use std::sync::Arc;

use anyhow::{Ok, Result};
use axum::async_trait;
use diesel::{
    dsl::insert_into,
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};

use crate::{
    domain::{
        entities::adventurers::{AdventurerEntity, RegisterAdventurerEntity},
        repositories::adventurers::AdventurersRepository,
    },
    infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::adventurers},
};

pub struct AdventurersPostgres {
    //  using Arc because it is pool and has a using on multi-thread on the project
    db_pool: Arc<PgPoolSquad>,
}

impl AdventurersPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AdventurersRepository for AdventurersPostgres {
    async fn register(&self, register_adventurer_entity: RegisterAdventurerEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(adventurers::table)
            .values(register_adventurer_entity)
            .returning(adventurers::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = adventurers::table
            .filter(adventurers::username.eq(username))
            .select(AdventurerEntity::as_select())
            .first(&mut conn)?;

        Ok(result)
    }
}
