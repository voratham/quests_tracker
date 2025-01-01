// @generated automatically by Diesel CLI.

diesel::table! {
    adventurers (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    guild_commanders (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    quest_adventurer_junction (quest_id, adventurer_id) {
        quest_id -> Int4,
        adventurer_id -> Int4,
    }
}

diesel::table! {
    quests (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        status -> Varchar,
        guild_commander_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(quest_adventurer_junction -> adventurers (adventurer_id));
diesel::joinable!(quest_adventurer_junction -> quests (quest_id));
diesel::joinable!(quests -> guild_commanders (guild_commander_id));

diesel::allow_tables_to_appear_in_same_query!(
    adventurers,
    guild_commanders,
    quest_adventurer_junction,
    quests,
);
