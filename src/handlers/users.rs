use axum::{http::StatusCode, response::IntoResponse, Json};
use diesel::result::DatabaseErrorKind;

use crate::{
    calculate_hash, establish_connection, models::users::NewUser, repositories,
    requests::users::CreateUserRequest,
};

pub async fn post_user(
    Json(request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    let key = uuid::Uuid::new_v4().to_string();
    let key_hash = calculate_hash(&key);

    let new_user = NewUser {
        username: &request.username,
        key_hash: &key_hash.to_string(),
    };

    match repositories::users::insert_user(conn, new_user) {
        Ok(user) => Ok(format!(
            "Welcome {}! Your api key is: {} Don't lose it!",
            user.username, key
        )
        .into_response()),
        Err(e) => {
            eprintln!("error inserting user: {}", e);
            match e {
                diesel::result::Error::DatabaseError(kind, _) => {
                    match kind {
                        DatabaseErrorKind::UniqueViolation => Ok((
                            StatusCode::UNPROCESSABLE_ENTITY,
                            format!("the username \"{}\" is already taken", &request.username),
                        )
                            .into_response()),
                        _ => Ok((StatusCode::INTERNAL_SERVER_ERROR, "error inserting user")
                            .into_response()),
                    }
                }
                _ => {
                    Ok((StatusCode::INTERNAL_SERVER_ERROR, "error inserting user").into_response())
                }
            }
        }
    }
}
