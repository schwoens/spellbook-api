use strum::VariantNames;
use thiserror::Error;

use crate::enums::MagicSchool;

#[derive(Debug, Error)]
pub enum SpellValidationError {
    #[error("invalid spell level \"{0}\" expected \"Cantrip\" or \"Level [1-9]\"")]
    InvalidSpellLevel(String),
    #[error(
        "invalid school of magic \"{0}\" expected one of: {:?}",
        MagicSchool::VARIANTS
    )]
    InvalidMagicSchool(String),
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Authentication failed")]
    AuthError,
}
