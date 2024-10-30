use diesel::prelude::*;
use serde::Serialize;

use crate::{
    resources::{IntoCollection, IntoResource, SpellResource},
    schema::spells,
};

#[derive(Queryable, Selectable, Serialize)]
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
}

impl IntoResource<SpellResource> for Spell {
    fn into_resource(self) -> SpellResource {
        SpellResource {
            name: self.name,
            level: self.level,
            casting_time: self.casting_time,
            magic_school: self.magic_school,
            concentration: self.concentration,
            range: self.range,
            duration: self.duration,
            description: self.description,
        }
    }
}

impl IntoCollection<SpellResource> for Vec<Spell> {
    fn into_collection(self) -> Vec<SpellResource> {
        self.into_iter()
            .map(|spell| spell.into_resource())
            .collect()
    }
}

#[derive(Insertable, AsChangeset)]
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
}
