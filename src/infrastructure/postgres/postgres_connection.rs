use anyhow::{Ok, Result};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

// declare a type alias for the connection pool
pub type PgPoolSquad = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> Result<PgPoolSquad> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?; // propagate the error
    Ok(pool)
}
