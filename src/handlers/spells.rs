use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use crate::{
    establish_connection,
    models::spells::{NewSpell, UpdatedSpell},
    repositories,
    requests::spells::{
        CreateSpellRequest, DeleteSpellRequest, GetPublicSpellRequest, GetSpellRequest,
        PublishSpellRequest, UpdateSpellRequest,
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
    Json(request): Json<GetSpellRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    match repositories::spells::get_spell(conn, user_id, &request.name) {
        Ok(spell) => Ok(Json(spell.into_resource()).into_response()),
        Err(e) => match e {
            diesel::result::Error::NotFound => Ok((
                StatusCode::NOT_FOUND,
                format!("a spell with the name \"{}\" does not exist", &request.name),
            )
                .into_response()),
            _ => {
                eprintln!("error retrieving spells: {}", e);
                Ok((StatusCode::INTERNAL_SERVER_ERROR, "error retrieving spells").into_response())
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

    if let Ok(spell) = repositories::spells::get_spell(conn, user_id, &request.name) {
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
    Json(request): Json<UpdateSpellRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    if let Err(e) = request.validate() {
        return Ok((StatusCode::BAD_REQUEST, e.to_string()).into_response());
    }

    if let Some(new_name) = &request.updated_spell.name {
        if let Ok(spell) = repositories::spells::get_spell(conn, user_id, new_name) {
            if spell.name.to_lowercase() != request.name.to_lowercase() {
                return Ok((
                    StatusCode::BAD_REQUEST,
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

    match repositories::spells::update_spell(conn, user_id, &request.name, updated_spell) {
        Ok(spell) => Ok(Json(spell.into_resource()).into_response()),
        Err(diesel::result::Error::NotFound) => Ok((
            StatusCode::NOT_FOUND,
            format!(
                "You don't have a spell with the name \"{}\" in your spellbook.",
                &request.name
            ),
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
    Json(request): Json<DeleteSpellRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    match repositories::spells::delete_spell(conn, user_id, &request.name) {
        Ok(1) => Ok((
            StatusCode::OK,
            format!(
                "The spell \"{}\" was successfully erased from your spellbook.",
                &request.name
            ),
        )
            .into_response()),
        Ok(_) => Ok((
            StatusCode::NOT_FOUND,
            format!(
                "You don't have a spell with the name \"{}\" in your spellbook.",
                &request.name
            ),
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
    Json(request): Json<PublishSpellRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    if repositories::spells::is_published(conn, user_id, &request.name).is_ok_and(|x| x) {
        return Ok((
            StatusCode::UNPROCESSABLE_ENTITY,
            format!("Your spell \"{}\" is already published.", &request.name),
        )
            .into_response());
    }

    match repositories::spells::publish_spell(conn, user_id, &request.name, true) {
        Ok(spell) => Ok((
            StatusCode::OK,
            format!("Your spell \"{}\" was successfully published.", spell.name),
        )
            .into_response()),
        Err(diesel::result::Error::NotFound) => Ok((
            StatusCode::NOT_FOUND,
            format!(
                "You don't have a spell with the name \"{}\" in your spellbook.",
                &request.name
            ),
        )
            .into_response()),
        Err(e) => {
            eprintln!("error publishing spell: {}", e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, "error publishing spell").into_response())
        }
    }
}

pub async fn unpublish_spell(
    Extension(user_id): Extension<i32>,
    Json(request): Json<PublishSpellRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection();

    if !repositories::spells::is_published(conn, user_id, &request.name).is_ok_and(|x| x) {
        return Ok((
            StatusCode::BAD_REQUEST,
            format!("Your spell \"{}\" is not published.", &request.name),
        )
            .into_response());
    }

    match repositories::spells::publish_spell(conn, user_id, &request.name, false) {
        Ok(spell) => Ok((
            StatusCode::OK,
            format!(
                "Your spell \"{}\" was successfully unpublished.",
                spell.name
            ),
        )
            .into_response()),
        Err(diesel::result::Error::NotFound) => Ok((
            StatusCode::NOT_FOUND,
            format!(
                "You don't have a spell with the name \"{}\" in your spellbook.",
                &request.name
            ),
        )
            .into_response()),
        Err(e) => {
            eprintln!("error unpublishing spell: {}", e);
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                "error unpublishing spell",
            )
                .into_response())
        }
    }
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
