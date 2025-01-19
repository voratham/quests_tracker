use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::{delete, dsl::insert_into, ExpressionMethods, RunQueryDsl};

use crate::{
    domain::{
        repositories::crew_switchboard::CrewSwitchboardRepository,
        value_objects::quest_adventurer_junction::QuestAdventurerJunction,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, schema::quest_adventurer_junction,
    },
};

pub struct CrewSwitchBoardPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewSwitchBoardPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CrewSwitchboardRepository for CrewSwitchBoardPostgres {
    async fn join(&self, junction_body: QuestAdventurerJunction) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        match insert_into(quest_adventurer_junction::table)
            .values(junction_body)
            .execute(&mut conn)
        {
            Ok(_) => Ok(()),
            Err(e) => {
                return match e {
                    diesel::result::Error::DatabaseError(
                        diesel::result::DatabaseErrorKind::UniqueViolation,
                        _,
                    ) => Err(anyhow::anyhow!("You already joined quest")),
                    _ => Err(anyhow::anyhow!(e.to_string())),
                }
            }
        }
    }
    async fn leave(&self, junction_body: QuestAdventurerJunction) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = delete(quest_adventurer_junction::table)
            .filter(quest_adventurer_junction::adventurer_id.eq(junction_body.adventurer_id))
            .filter(quest_adventurer_junction::quest_id.eq(junction_body.quest_id))
            .execute(&mut conn)?;

        if result == 0 {
            return Err(anyhow::anyhow!("Failed to leave quest"));
        }

        Ok(())
    }
}
