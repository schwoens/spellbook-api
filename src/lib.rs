use std::{
    env,
    error::Error,
    hash::{DefaultHasher, Hash, Hasher},
};

use axum::http::HeaderMap;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use errors::AuthError;

pub mod enums;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod requests;
pub mod resources;
pub mod schema;
pub mod validators;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("environment variable DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}

pub trait Validate<T: Error> {
    fn validate(&self) -> Result<(), T>;
}

pub trait IntoResource<T> {
    fn into_resource(self) -> T;
}

pub trait IntoCollection<T> {
    fn into_collection(self) -> Vec<T>;
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

pub fn authenticate(conn: &mut PgConnection, headers: HeaderMap) -> Result<i32, AuthError> {
    if !headers.contains_key("key") {
        return Err(AuthError::AuthError);
    }
    let api_key = headers["key"].to_str().unwrap();

    let key_hash = calculate_hash(&api_key).to_string();
    match repositories::users::get_users(conn) {
        Ok(users) => match users.iter().find(|u| u.key_hash == key_hash) {
            Some(user) => Ok(user.id),
            None => Err(AuthError::AuthError),
        },
        Err(_) => Err(AuthError::AuthError),
    }
}
