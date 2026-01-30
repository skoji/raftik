use super::{error::Error, store::Store};
use crate::{
    ast::{
        ModuleParsed,
        instructions::Opcode,
        section::{CodeSection, FunctionSection, TypeSection},
        types::{FunctionType, ValueType},
    },
    validation::validate_module,
};

#[derive(Debug, Clone, Default)]
pub struct Module {
    pub func_types: Vec<FunctionType>,
    pub func_addresses: Vec<usize>,
}

impl Module {
    pub fn from_slice(wat: &[u8], store: &mut Store) -> Result<Self, Error> {
        let module_parsed =
            ModuleParsed::from_slice(wat).map_err(|e| Error::ParseError { parse_error: e })?;
        validate_module(&module_parsed).map_err(|e| Error::ValidationError {
            validation_error: e,
        })?;
        let module = Module::default();
        Ok(module)
    }
}

#[derive(Debug, Clone)]
pub struct WasmFunc {
    pub t: FunctionType,
    pub module_address: usize,
    pub locals: Vec<ValueType>,
    pub body: Vec<Opcode>,
}

#[derive(Debug, Clone)]
pub struct ExternalFunc {
    pub t: FunctionType,
    // TODO; expression of body implementation
}

#[derive(Debug, Clone)]
pub enum Func {
    Wasm(WasmFunc),
    External(ExternalFunc),
}

#[derive(Debug, Clone)]
pub struct Export {
    pub name: String,
    pub value: ExportValue,
}

#[derive(Debug, Clone)]
pub enum ExportValue {
    Func(usize),
}
