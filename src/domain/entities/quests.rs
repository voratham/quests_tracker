use chrono::NaiveDateTime;
use diesel::prelude::*;

// import from schema.rs on infrastructure/postgres
use crate::infrastructure::postgres::schema::quests;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = quests)]
pub struct QuestEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub guild_commander_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = quests)]
pub struct AddQuestEntity {
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub guild_commander_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// AsChangeset options is if when create or update data it will ignore if field = none

#[derive(Debug, Clone, Queryable, AsChangeset)]
#[diesel(table_name = quests)]
pub struct EditQuestEntity {
    pub name: String,
    pub description: Option<String>,
    pub guild_commander_id: i32,
    pub updated_at: NaiveDateTime,
}
