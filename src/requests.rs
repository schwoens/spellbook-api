use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateSpellRequest {
    pub name: String,
    pub level: i32,
    pub time: String,
    pub school: String,
    pub concentration: bool,
    pub range: String,
    pub duration: String,
}

pub struct UpdateSpellRequest {
    pub name: String,
    pub updated_spell: CreateSpellRequest,
}
