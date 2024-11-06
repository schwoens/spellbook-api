use std::str::FromStr;

use regex::Regex;
use serde::Deserialize;

use crate::{enums::MagicSchool, errors::SpellValidationError};

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

impl UpdatedSpellData {
    pub fn validate(&self) -> Result<(), SpellValidationError> {
        if let Some(level) = &self.level {
            let level_regex = Regex::new("^Level [1-9]$|^Cantrip$").unwrap();
            if !level_regex.is_match(level) {
                return Err(SpellValidationError::InvalidSpellLevel(level.to_string()));
            }
        }

        if let Some(magic_school) = &self.magic_school {
            if MagicSchool::from_str(magic_school).is_err() {
                return Err(SpellValidationError::InvalidMagicSchool(
                    magic_school.to_string(),
                ));
            }
        }

        Ok(())
    }
}

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
