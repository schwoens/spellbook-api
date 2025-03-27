use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use nanoid::nanoid;

use crate::{
    establish_connection,
    models::spells::{NewSpell, UpdatedSpell},
    repositories,
    requests::spells::{
        CreateSpellRequest, QueryPublicSpellsRequest, QuerySpellsRequest, UpdateSpellRequest,
    },
    IntoCollection, IntoResource, Validate,
};

pub async fn get_spells(
    Extension(user_id): Extension<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();
    match repositories::spells::get_spells(conn, user_id) {
        Ok(spells) => Ok(Json(spells.into_collection()).into_response()),
        Err(e) => {
            let msg = "Failed to retrieve spells";
            eprintln!("{}: {}", msg, e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, msg).into_response())
        }
    }
}

pub async fn get_spell(
    Extension(user_id): Extension<i32>,
    Path(nanoid): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    match repositories::spells::get_spell_by_nanoid(conn, user_id, &nanoid) {
        Ok(spell) => Ok(Json(spell.into_resource()).into_response()),
        Err(e) => match e {
            diesel::result::Error::NotFound => Ok((
                StatusCode::NOT_FOUND,
                format!(
                    "You don't have a spell with the id \"{}\" in your spellbook",
                    nanoid
                ),
            )
                .into_response()),
            _ => {
                let msg = "Failed to retrieve spell";
                eprintln!("{}: {}", msg, e);
                Ok((StatusCode::INTERNAL_SERVER_ERROR, msg).into_response())
            }
        },
    }
}

pub async fn post_spell(
    Extension(user_id): Extension<i32>,
    Json(request): Json<CreateSpellRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    if let Err(e) = request.validate() {
        return Ok((StatusCode::UNPROCESSABLE_ENTITY, e.to_string()).into_response());
    }

    // check if a spell with that name already exists in this users' spellbook
    if let Ok(spell) = repositories::spells::get_spell_by_name(conn, user_id, &request.name) {
        return Ok((
            StatusCode::UNPROCESSABLE_ENTITY,
            format!(
                "You already have a spell with the name \"{}\" in your spellbook.",
                spell.name
            ),
        )
            .into_response());
    }

    let new_spell = NewSpell {
        name: &request.name,
        level: &request.level,
        casting_time: &request.casting_time,
        magic_school: &request.magic_school,
        concentration: request.concentration,
        range: &request.range,
        duration: &request.duration,
        description: &request.description,
        user_id,
        published: false,
        nanoid: &nanoid!(),
    };

    match repositories::spells::insert_spell(conn, new_spell) {
        Ok(spell) => Ok(Json(spell.into_resource()).into_response()),
        Err(e) => {
            let msg = "Failed to insert spell";
            eprintln!("{}: {}", msg, e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, msg).into_response())
        }
    }
}

pub async fn update_spell(
    Extension(user_id): Extension<i32>,
    Path(nanoid): Path<String>,
    Json(request): Json<UpdateSpellRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    if let Err(e) = request.validate() {
        return Ok((StatusCode::UNPROCESSABLE_ENTITY, e.to_string()).into_response());
    }

    if let Some(new_name) = &request.name {
        if let Ok(spell) = repositories::spells::get_spell_by_name(conn, user_id, new_name) {
            if spell.nanoid != nanoid {
                return Ok((
                    StatusCode::UNPROCESSABLE_ENTITY,
                    format!(
                        "You already have a spell with the name \"{}\" in your spellbook.",
                        spell.name
                    ),
                )
                    .into_response());
            }
        }
    }

    let updated_spell = UpdatedSpell::from_request(&request);

    match repositories::spells::update_spell(conn, user_id, &nanoid, updated_spell) {
        Ok(spell) => Ok(Json(spell.into_resource()).into_response()),
        Err(diesel::result::Error::NotFound) => Ok((
            StatusCode::NOT_FOUND,
            format!(
                "You don't have a spell with the id \"{}\" in your spellbook.",
                nanoid
            ),
        )
            .into_response()),
        Err(e) => {
            let msg = "Failed to update spell";
            eprintln!("{}: {}", msg, e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, msg).into_response())
        }
    }
}

pub async fn delete_spell(
    Extension(user_id): Extension<i32>,
    Path(nanoid): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    match repositories::spells::delete_spell(conn, user_id, &nanoid) {
        Ok(1) => Ok((
            StatusCode::OK,
            "The spell was successfully erased from your spellbook",
        )
            .into_response()),
        Ok(_) => Ok((
            StatusCode::NOT_FOUND,
            format!(
                "You don't have a spell with the id \"{}\" in your spellbook.",
                nanoid
            ),
        )
            .into_response()),
        Err(e) => {
            let msg = "Failed to erase spell";
            eprintln!("{}: {}", msg, e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, msg).into_response())
        }
    }
}

pub async fn publish_spell(
    Extension(user_id): Extension<i32>,
    Path(nanoid): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    match repositories::spells::get_spell_by_nanoid(conn, user_id, &nanoid) {
        Ok(spell) => {
            if spell.published {
                return Ok((
                    StatusCode::UNPROCESSABLE_ENTITY,
                    format!("Your spell \"{}\" is already published", &spell.name),
                ));
            }
            match repositories::spells::publish_spell(conn, user_id, &nanoid, true) {
                Ok(_) => Ok((
                    StatusCode::OK,
                    format!("Your spell \"{}\" was successfully published", &spell.name),
                )),
                Err(e) => {
                    let msg = "Failed to publish spell".to_string();
                    eprintln!("{}: {}", msg, e);
                    Ok((StatusCode::INTERNAL_SERVER_ERROR, msg))
                }
            }
        }
        Err(diesel::result::Error::NotFound) => Ok((
            StatusCode::NOT_FOUND,
            format!(
                "You don't have a spell with the id \"{}\" in your spellbook",
                nanoid
            ),
        )),
        Err(e) => {
            let msg = "Failed to retrieve spell".to_string();
            eprintln!("{}: {}", msg, e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, msg))
        }
    }
}

pub async fn unpublish_spell(
    Extension(user_id): Extension<i32>,
    Path(nanoid): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    match repositories::spells::get_spell_by_nanoid(conn, user_id, &nanoid) {
        Ok(spell) => {
            if !spell.published {
                return Ok((
                    StatusCode::UNPROCESSABLE_ENTITY,
                    format!("Your spell \"{}\" is not public", &spell.name),
                ));
            }
            match repositories::spells::publish_spell(conn, user_id, &nanoid, false) {
                Ok(_) => Ok((
                    StatusCode::OK,
                    format!(
                        "Your spell \"{}\" was successfully unpublished",
                        &spell.name
                    ),
                )),
                Err(e) => {
                    let msg = "Failed to unpublish spell".to_string();
                    eprintln!("{}: {}", msg, e);
                    Ok((StatusCode::INTERNAL_SERVER_ERROR, msg))
                }
            }
        }
        Err(diesel::result::Error::NotFound) => Ok((
            StatusCode::NOT_FOUND,
            format!(
                "You don't have a spell with the id \"{}\" in your spellbook",
                nanoid
            ),
        )),
        Err(e) => {
            let msg = "Failed to retrieve spell".to_string();
            eprintln!("{}: {}", msg, e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, msg))
        }
    }
}

pub async fn query_spells(
    Extension(user_id): Extension<i32>,
    Json(request): Json<QuerySpellsRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    match repositories::spells::query_spells(conn, user_id, request) {
        Ok(spells) => Ok(Json(spells.into_collection()).into_response()),
        Err(e) => {
            let msg = "Failed to query spells";
            eprintln!("{}: {}", msg, e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, msg).into_response())
        }
    }
}

pub async fn query_public_spells(
    Extension(user_id): Extension<i32>,
    Json(request): Json<QueryPublicSpellsRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    match repositories::spells::query_public_spells(conn, user_id, request) {
        Ok(spells_with_users) => Ok(Json(spells_with_users.into_collection()).into_response()),
        Err(e) => {
            let msg = "Failed to retrieve public spells";
            eprintln!("{}: {}", msg, e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, msg).into_response())
        }
    }
}

pub async fn copy_public_spell(
    Extension(user_id): Extension<i32>,
    Path(nanoid): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    let spell = repositories::spells::get_public_spell(conn, &nanoid);

    match spell {
        Err(e) => match e {
            diesel::result::Error::NotFound => Ok((
                StatusCode::NOT_FOUND,
                format!("A public spell with the id \"{}\" does not exist", nanoid),
            )
                .into_response()),
            _ => {
                let msg = "Failed to retrieve published spell";
                eprintln!("{}: {}", msg, e);
                Ok((StatusCode::INTERNAL_SERVER_ERROR, msg).into_response())
            }
        },
        Ok(spell) => {
            if let Ok(spell) = repositories::spells::get_spell_by_name(conn, user_id, &spell.name) {
                Ok((
                    StatusCode::UNPROCESSABLE_ENTITY,
                    format!(
                        "You already have a spell with the name \"{}\" in your spellbook",
                        spell.name
                    ),
                )
                    .into_response())
            } else {
                let copy = NewSpell {
                    name: &spell.name,
                    level: &spell.level,
                    casting_time: &spell.casting_time,
                    magic_school: &spell.magic_school,
                    concentration: spell.concentration,
                    range: &spell.range,
                    duration: &spell.duration,
                    description: &spell.description,
                    user_id,
                    published: false,
                    nanoid: &nanoid!(),
                };

                match repositories::spells::insert_spell(conn, copy) {
                    Ok(spell) => Ok(Json(spell.into_resource()).into_response()),
                    Err(e) => {
                        let msg = "Failed to copy spell";
                        eprintln!("{}: {}", msg, e);
                        Ok((StatusCode::INTERNAL_SERVER_ERROR, msg).into_response())
                    }
                }
            }
        }
    }
}
