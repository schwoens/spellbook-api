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
    pub level: i32,
    pub time: String,
    pub school: String,
    pub concentration: bool,
    pub range: String,
    pub duration: String,
}

impl IntoResource<SpellResource> for Spell {
    fn into_resource(self) -> SpellResource {
        let level = match self.level {
            0 => "Cantrip".to_string(),
            l => l.to_string(),
        };
        SpellResource {
            name: self.name,
            level,
            time: self.time,
            school: self.school,
            concentration: self.concentration,
            range: self.range,
            duration: self.duration,
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

#[derive(Insertable)]
#[diesel(table_name = spells)]
pub struct NewSpell<'a> {
    pub name: &'a str,
    pub level: i32,
    pub time: &'a str,
    pub school: &'a str,
    pub concentration: bool,
    pub range: &'a str,
    pub duration: &'a str,
}
