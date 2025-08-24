use crate::ast::ModuleParsed;
pub mod error;
use error::Error;

pub fn validate_module(_module: &ModuleParsed) -> Result<(), Error> {
    Ok(())
}
