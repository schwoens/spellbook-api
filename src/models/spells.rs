use diesel::prelude::*;

use crate::{requests::spells::UpdateSpellRequest, schema::spells};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::spells)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Spell {
    pub id: i32,
    pub name: String,
    pub level: String,
    pub casting_time: String,
    pub magic_school: String,
    pub concentration: bool,
    pub range: String,
    pub duration: String,
    pub description: String,
    pub user_id: i32,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = spells)]
pub struct NewSpell<'a> {
    pub name: &'a str,
    pub level: &'a str,
    pub casting_time: &'a str,
    pub magic_school: &'a str,
    pub concentration: bool,
    pub range: &'a str,
    pub duration: &'a str,
    pub description: &'a str,
    pub user_id: i32,
    pub published: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = spells)]
#[diesel(belongs_to(users))]
pub struct UpdatedSpell<'a> {
    pub name: Option<&'a str>,
    pub level: Option<&'a str>,
    pub casting_time: Option<&'a str>,
    pub magic_school: Option<&'a str>,
    pub concentration: Option<bool>,
    pub range: Option<&'a str>,
    pub duration: Option<&'a str>,
    pub description: Option<&'a str>,
}

impl<'a> UpdatedSpell<'a> {
    pub fn from_request(request: &'a UpdateSpellRequest) -> Self {
        UpdatedSpell {
            name: request.updated_spell.name.as_deref(),
            level: request.updated_spell.level.as_deref(),
            casting_time: request.updated_spell.casting_time.as_deref(),
            magic_school: request.updated_spell.magic_school.as_deref(),
            concentration: request.updated_spell.concentration,
            range: request.updated_spell.range.as_deref(),
            duration: request.updated_spell.duration.as_deref(),
            description: request.updated_spell.description.as_deref(),
        }
    }
}
