use crate::validation::error::ValidationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("parse error:{parse_error}")]
    ParseError { parse_error: String },

    #[error("validation error:{validation_error}")]
    ValidationError { validation_error: ValidationError },
}
