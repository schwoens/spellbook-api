use std::env;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

pub mod models;
pub mod repositories;
pub mod requests;
pub mod resources;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("environment variable DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
