use super::error::Error;
use super::{rawinstance::Module as RawModule, store::Store};
use crate::{ast::ModuleParsed, validation::validate_module};

#[derive(Debug, Clone, Default)]
pub struct Module {
    index: usize,
}

impl Module {
    pub fn from_slice(wat: &[u8], store: &mut Store) -> Result<Self, Error> {
        let module =
            ModuleParsed::from_slice(wat).map_err(|e| Error::ParseError { parse_error: e })?;
        validate_module(&module).map_err(|e| Error::ValidationError {
            validation_error: e,
        })?;
        let raw_module = RawModule::default();
        let index = store.register_module(raw_module);
        Ok(Module { index })
    }
}
