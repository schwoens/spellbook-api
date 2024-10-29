use axum::{http::StatusCode, response::IntoResponse, routing::get};
use axum::{Json, Router};
use diesel::result::DatabaseErrorKind;
use spellbook_api::{
    establish_connection,
    models::NewSpell,
    repositories::{self, spells::UpdatedSpell},
    requests::{CreateSpellRequest, DeleteSpellRequest, UpdateSpellRequest},
    resources::{IntoCollection, IntoResource},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route(
            "/spells",
            get(get_spells)
                .post(post_spell)
                .put(update_spell)
                .delete(delete_spell),
        );

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
        casting_time: &request.casting_time,
        magic_school: &request.magic_school,
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

async fn update_spell(
    Json(request): Json<UpdateSpellRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    if let Err(e) = request.validate() {
        return Ok((StatusCode::BAD_REQUEST, e.to_string()).into_response());
    }

    let updated_spell = UpdatedSpell {
        name: &request.updated_spell.name,
        level: &request.updated_spell.level,
        casting_time: &request.updated_spell.casting_time,
        magic_school: &request.updated_spell.magic_school,
        concentration: request.updated_spell.concentration,
        range: &request.updated_spell.range,
        duration: &request.updated_spell.duration,
    };

    match repositories::spells::update_spell(conn, &request.name, updated_spell) {
        Ok(spell) => Ok(Json(spell.into_resource()).into_response()),
        Err(e) => {
            eprintln!("error updating spell: {}", e);
            match e {
                diesel::result::Error::DatabaseError(kind, _) => {
                    match kind {
                        DatabaseErrorKind::UniqueViolation => Ok((
                            StatusCode::BAD_REQUEST,
                            format!("a spell with the name \"{}\" already exists", &request.name),
                        )
                            .into_response()),
                        _ => Ok((StatusCode::INTERNAL_SERVER_ERROR, "error updating spell")
                            .into_response()),
                    }
                }
                diesel::result::Error::NotFound => Ok((
                    StatusCode::NOT_FOUND,
                    format!("a spell with the name \"{}\" does not exist", &request.name),
                )
                    .into_response()),
                _ => {
                    Ok((StatusCode::INTERNAL_SERVER_ERROR, "error updating spell").into_response())
                }
            }
        }
    }
}

async fn delete_spell(
    Json(request): Json<DeleteSpellRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    match repositories::spells::delete_spell(conn, &request.name) {
        Ok(1) => Ok((
            StatusCode::OK,
            format!("the spell \"{}\" was successfully deleted", &request.name),
        )
            .into_response()),
        Ok(_) => Ok((
            StatusCode::NOT_FOUND,
            format!("a spell with the name \"{}\" does not exist", &request.name),
        )
            .into_response()),
        Err(e) => {
            eprintln!("error deleting spell: {}", e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, "error deleting spell").into_response())
        }
    }
}
