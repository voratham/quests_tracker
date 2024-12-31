use super::{
    config_model::{AdventurerSecret, Database, DotEnvyConfig, GuildCommanderSecret, Server},
    stage::Stage,
};
use anyhow::{Ok, Result};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is invalid")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is invalid")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is invalid")
            .parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    Ok(DotEnvyConfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());

    Stage::try_form(&stage_str).unwrap_or_default()
}

pub fn get_adventurer_secret_env() -> Result<AdventurerSecret> {
    dotenvy::dotenv().ok();

    Ok(AdventurerSecret {
        secret: std::env::var("JWT_ADVENTURER_SECRET")
            .expect("JWT_ADVENTURER_SECRET is invalid")
            .parse()?,
        refresh_secret: std::env::var("JWT_ADVENTURER_REFRESH_SECRET")
            .expect("JWT_ADVENTURER_REFRESH_SECRET is invalid")
            .parse()?,
    })
}

pub fn get_guild_commanders_secret_env() -> Result<GuildCommanderSecret> {
    dotenvy::dotenv().ok();

    Ok(GuildCommanderSecret {
        secret: std::env::var("JWT_GUILD_COMMANDER_SECRET")
            .expect("JWT_GUILD_COMMANDER_SECRET is invalid")
            .parse()?,
        refresh_secret: std::env::var("JWT_GUILD_COMMANDER_REFRESH_SECRET")
            .expect("JWT_GUILD_COMMANDER_REFRESH_SECRET is invalid")
            .parse()?,
    })
}
