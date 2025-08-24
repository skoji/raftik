use crate::ast::Module;
pub mod error;
use error::Error;

pub fn validate_module(_module: &Module) -> Result<(), Error> {
    Ok(())
}
