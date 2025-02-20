use axum::{
    extract::Request,
    http::{self, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{calculate_hash, establish_connection, repositories};

pub async fn auth(mut request: Request, next: Next) -> Response {
    let conn = &mut establish_connection();
    let auth_header = request.headers().get(http::header::AUTHORIZATION);

    let header_value = match auth_header {
        Some(value) => value,
        None => return (StatusCode::UNAUTHORIZED, "Missing AUTHORIZATION header").into_response(),
    };

    let api_key = match header_value.to_str() {
        Ok(api_key) => api_key.to_string(),
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                "Invalid AUTHORIZATION header value",
            )
                .into_response()
        }
    };

    let key_hash = calculate_hash(&api_key).to_string();

    let user = match repositories::users::get_user_by_key_hash(conn, key_hash) {
        Ok(user) => user,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                "A user with this api key does not exist",
            )
                .into_response()
        }
    };

    request.extensions_mut().insert(user.id);
    next.run(request).await
}
