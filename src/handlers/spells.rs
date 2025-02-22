use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use nanoid::nanoid;

use crate::{
    establish_connection,
    models::spells::{NewSpell, UpdatedSpell},
    repositories,
    requests::spells::{
        CreateSpellRequest, GetPublicSpellRequest, QuerySpellRequest, UpdateSpellRequest,
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
            eprintln!("error retrieving spells: {}", e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, "error retrieving spells").into_response())
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
                "You don't have a spell with this id in your spellbook.",
            )
                .into_response()),
            _ => {
                eprintln!("error retrieving spell: {}", e);
                Ok((StatusCode::INTERNAL_SERVER_ERROR, "error retrieving spell").into_response())
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
            eprintln!("error inserting spell: {}", e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, "error inserting spell").into_response())
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

    match repositories::spells::update_spell(conn, user_id, nanoid, updated_spell) {
        Ok(spell) => Ok(Json(spell.into_resource()).into_response()),
        Err(diesel::result::Error::NotFound) => Ok((
            StatusCode::NOT_FOUND,
            "You don't have a spell with this id in your spellbook.",
        )
            .into_response()),
        Err(e) => {
            eprintln!("error updating spell: {}", e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, "error updating spell").into_response())
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
            "The spell was successfully erased from your spellbook.",
        )
            .into_response()),
        Ok(_) => Ok((
            StatusCode::NOT_FOUND,
            "You don't have a spell with this id in your spellbook.",
        )
            .into_response()),
        Err(e) => {
            eprintln!("error deleting spell: {}", e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, "error deleting spell").into_response())
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
                    format!("Your spell \"{}\" is already published.", &spell.name),
                ));
            }
            match repositories::spells::publish_spell(conn, user_id, &nanoid, true) {
                Ok(_) => Ok((
                    StatusCode::OK,
                    format!("Your spell \"{}\" was successfully published.", &spell.name),
                )),
                Err(e) => {
                    let msg = format!("error publishing spell: {}", e);
                    eprintln!("{}", msg);
                    Ok((StatusCode::INTERNAL_SERVER_ERROR, msg))
                }
            }
        }
        Err(diesel::result::Error::NotFound) => Ok((
            StatusCode::NOT_FOUND,
            format!(
                "A spell with the id \"{}\" does not exist in your spellbook.",
                nanoid
            ),
        )),
        Err(e) => {
            let msg = format!("error fetching spell: {}", e);
            eprintln!("{}", msg);
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
                    format!("Your spell \"{}\" is not public.", &spell.name),
                ));
            }
            match repositories::spells::publish_spell(conn, user_id, &nanoid, false) {
                Ok(_) => Ok((
                    StatusCode::OK,
                    format!(
                        "Your spell \"{}\" was successfully unpublished.",
                        &spell.name
                    ),
                )),
                Err(e) => {
                    let msg = format!("error unpublishing spell: {}", e);
                    eprintln!("{}", msg);
                    Ok((StatusCode::INTERNAL_SERVER_ERROR, msg))
                }
            }
        }
        Err(diesel::result::Error::NotFound) => Ok((
            StatusCode::NOT_FOUND,
            format!(
                "A spell with the id \"{}\" does not exist in your spellbook.",
                nanoid
            ),
        )),
        Err(e) => {
            let msg = format!("error fetching spell: {}", e);
            eprintln!("{}", msg);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, msg))
        }
    }
}

pub async fn query_spells(
    Extension(user_id): Extension<i32>,
    Json(request): Json<QuerySpellRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    match repositories::spells::query_spells(conn, &request) {}
}

pub async fn query_public_spells(
    Json(request): Json<GetPublicSpellRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    match repositories::spells::query_public_spells(conn, &request.keyword) {
        Ok(spells_with_users) => Ok(Json(spells_with_users.into_collection()).into_response()),
        Err(e) => {
            eprintln!("error querying public spells: {}", e);
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                "error querying public spells",
            )
                .into_response())
        }
    }
}
