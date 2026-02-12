use super::{error::Error, store::Store};
use crate::{
    ast::{
        ModuleParsed,
        instructions::Opcode,
        section::{FunctionBody, Import, Section, SectionID},
        types::{FunctionType, ValueType},
    },
    validation::validate_module,
};

#[derive(Debug, Clone, Default)]
pub struct Module {
    pub func_types: Vec<FunctionType>,
    pub func_addresses: Vec<usize>,
}

// workspace for build module
#[derive(Debug, Default)]
struct Sections<'a> {
    pub code: Vec<&'a FunctionBody<'a>>,
    pub functions: Vec<u32>, // type indicies
    pub imports: Vec<&'a Import>,
    pub types: Vec<&'a FunctionType>,
}

impl<'a> Sections<'a> {
    pub fn create_from_parsed_module(module_parsed: &'a ModuleParsed<'a>) -> Self {
        let mut r = Self::default();
        for section in module_parsed.sections.iter() {
            match section {
                Section::Code(code_section) => {
                    r.code = code_section.code.iter().collect();
                }
                Section::Function(function_section) => {
                    r.functions = function_section.type_indices.clone();
                }
                Section::Import(import_section) => {
                    r.imports = import_section.imports.iter().collect();
                }
                Section::Type(type_section) => {
                    r.types = type_section.types.iter().collect();
                }
                _ => (),
            }
        }
        r
    }
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

    fn build_from_parsed(module_parsed: &ModuleParsed, store: &mut Store) -> Result<Self, Error> {
        // parse module parsed and build store, and module
        let mut r = Module::default();
        let sections = Sections::create_from_parsed_module(module_parsed);
        Ok(r)
    }

    fn process_imports(&mut self, sections: &Sections, store: &mut Store) {}
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
