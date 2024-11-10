use serde::Serialize;

use crate::{
    models::{spells::Spell, users::User},
    IntoCollection, IntoResource,
};

#[derive(Serialize)]
pub struct SpellResource {
    pub name: String,
    pub level: String,
    pub casting_time: String,
    pub magic_school: String,
    pub concentration: bool,
    pub range: String,
    pub duration: String,
    pub description: String,
    pub published: bool,
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
            published: self.published,
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

#[derive(Serialize)]
pub struct PublicSpellResource {
    pub name: String,
    pub level: String,
    pub casting_time: String,
    pub magic_school: String,
    pub concentration: bool,
    pub range: String,
    pub duration: String,
    pub description: String,
    pub username: String,
}

impl IntoResource<PublicSpellResource> for (Spell, User) {
    fn into_resource(self) -> PublicSpellResource {
        PublicSpellResource {
            name: self.0.name,
            level: self.0.level,
            casting_time: self.0.casting_time,
            magic_school: self.0.magic_school,
            concentration: self.0.concentration,
            range: self.0.range,
            duration: self.0.duration,
            description: self.0.description,
            username: self.1.username,
        }
    }
}

impl IntoCollection<PublicSpellResource> for Vec<(Spell, User)> {
    fn into_collection(self) -> Vec<PublicSpellResource> {
        self.into_iter()
            .map(|spell_with_user| spell_with_user.into_resource())
            .collect()
    }
}
