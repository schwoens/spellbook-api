use axum::{http::StatusCode, response::IntoResponse, routing::get};
use axum::{Json, Router};
use diesel::result::DatabaseErrorKind;
use spellbook_api::{
    establish_connection,
    models::NewSpell,
    repositories,
    requests::CreateSpellRequest,
    resources::{IntoCollection, IntoResource},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/spells", get(get_spells).post(post_spell));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_spells() -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    match repositories::spells::get_spells(conn) {
        Ok(spells) => Ok(Json(spells.into_collection()).into_response()),
        Err(e) => {
            eprintln!("error retrieving spells: {}", e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, "error retrieving spells").into_response())
        }
    }
}

async fn post_spell(
    Json(request): Json<CreateSpellRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    if let Err(e) = request.validate() {
        return Ok((StatusCode::BAD_REQUEST, e.to_string()).into_response());
    }

    let new_spell = NewSpell {
        name: &request.name,
        level: &request.level,
        time: &request.time,
        school: &request.school,
        concentration: request.concentration,
        range: &request.range,
        duration: &request.duration,
    };

    match repositories::spells::insert_spell(conn, new_spell) {
        Ok(spell) => Ok(Json(spell.into_resource()).into_response()),
        Err(e) => {
            eprintln!("error inserting spell: {}", e);
            match e {
                diesel::result::Error::DatabaseError(kind, _) => {
                    if let DatabaseErrorKind::UniqueViolation = kind {
                        Ok((
                            StatusCode::BAD_REQUEST,
                            format!("a spell with the name \"{}\" already exists", request.name),
                        )
                            .into_response())
                    } else {
                        Ok((StatusCode::INTERNAL_SERVER_ERROR, "error inserting spell")
                            .into_response())
                    }
                }
                _ => Ok(
                    (StatusCode::INTERNAL_SERVER_ERROR, "error inserting spell").into_response()
                ),
            }
        }
    }
}
