use std::str::FromStr;

use regex::Regex;
use serde::Deserialize;

use crate::{enums::MagicSchool, errors::SpellValidationError};

#[derive(Deserialize)]
pub struct CreateSpellRequest {
    pub name: String,
    pub level: String,
    pub casting_time: String,
    pub magic_school: String,
    pub concentration: bool,
    pub range: String,
    pub duration: String,
}

impl CreateSpellRequest {
    pub fn validate(&self) -> Result<(), SpellValidationError> {
        let level_regex = Regex::new("^Level [1-9]$|^Cantrip$").unwrap();
        if !level_regex.is_match(&self.level) {
            return Err(SpellValidationError::InvalidSpellLevel(
                self.level.to_string(),
            ));
        }
        if MagicSchool::from_str(&self.magic_school).is_err() {
            return Err(SpellValidationError::InvalidMagicSchool(
                self.magic_school.to_string(),
            ));
        }
        Ok(())
    }
}

type UpdatedSpellData = CreateSpellRequest;

#[derive(Deserialize)]
pub struct UpdateSpellRequest {
    pub name: String,
    pub updated_spell: UpdatedSpellData,
}

impl UpdateSpellRequest {
    pub fn validate(&self) -> Result<(), SpellValidationError> {
        self.updated_spell.validate()
    }
}

#[derive(Deserialize)]
pub struct DeleteSpellRequest {
    pub name: String,
}
