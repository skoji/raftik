use super::error::Error;
use super::{rawinstance::Module as RawModule, store::Store};

#[derive(Debug, Clone, Default)]
pub struct Module {
    index: usize,
}

impl Module {
    pub fn from_slice(wat: &[u8], store: &mut Store) -> Result<Self, Error> {
        let index = store.register_module_from_wat(wat)?;
        Ok(Module { index })
    }
}
