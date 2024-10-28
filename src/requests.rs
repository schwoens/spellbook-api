use regex::Regex;
use serde::Deserialize;

use crate::errors::CreateSpellRequestValidationError;

#[derive(Deserialize)]
pub struct CreateSpellRequest {
    pub name: String,
    pub level: String,
    pub time: String,
    pub school: String,
    pub concentration: bool,
    pub range: String,
    pub duration: String,
}

impl CreateSpellRequest {
    pub fn validate(&self) -> Result<(), CreateSpellRequestValidationError> {
        let level_regex = Regex::new("^Level [1-9]$|^Cantrip$").unwrap();
        if !level_regex.is_match(&self.level) {
            return Err(CreateSpellRequestValidationError::InvalidSpellLevel(
                &self.level,
            ));
        }
        Ok(())
    }
}

pub struct UpdateSpellRequest {
    pub name: String,
    pub updated_spell: CreateSpellRequest,
}
