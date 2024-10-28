use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateSpellRequestValidationError<'a> {
    #[error("invalid spell level \"{0}\" expected \"Cantrip\" or \"Level [1-9]\"")]
    InvalidSpellLevel(&'a str),
}
