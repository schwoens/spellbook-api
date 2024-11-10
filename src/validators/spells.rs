use std::str::FromStr;

use regex::Regex;

use crate::{
    enums::MagicSchool,
    errors::SpellValidationError,
    requests::spells::{CreateSpellRequest, UpdatedSpellData},
    Validate,
};

impl Validate<SpellValidationError> for CreateSpellRequest {
    fn validate(&self) -> Result<(), SpellValidationError> {
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

impl Validate<SpellValidationError> for UpdatedSpellData {
    fn validate(&self) -> Result<(), SpellValidationError> {
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
