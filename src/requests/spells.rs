use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetSpellRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateSpellRequest {
    pub name: String,
    pub level: String,
    pub casting_time: String,
    pub magic_school: String,
    pub concentration: bool,
    pub range: String,
    pub duration: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdatedSpellData {
    pub name: Option<String>,
    pub level: Option<String>,
    pub casting_time: Option<String>,
    pub magic_school: Option<String>,
    pub concentration: Option<bool>,
    pub range: Option<String>,
    pub duration: Option<String>,
    pub description: Option<String>,
}

pub type UpdateSpellRequest = UpdatedSpellData;

#[derive(Deserialize)]
pub struct DeleteSpellRequest {
    pub name: String,
}

pub type PublishSpellRequest = DeleteSpellRequest;

#[derive(Deserialize)]
pub struct GetPublicSpellRequest {
    pub keyword: String,
}
