use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passport {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub role: Roles,
    pub exp: usize, //  expire token
    pub iat: usize, //  created at token
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Roles {
    Adventurer,
    GuildCommander,
}
