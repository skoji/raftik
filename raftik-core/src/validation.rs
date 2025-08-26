pub mod error;
mod section;

use crate::ast::{
    ModuleParsed, Section,
    types::{FunctionType, GlobalType, MemoryType, TableType, ValueType},
};

use error::ValidationError;

#[derive(Default)]
struct Context<'a> {
    pub types: Vec<&'a FunctionType>,
    pub functions: Vec<&'a u32>,
    pub tables: Vec<&'a TableType>,
    pub memories: Vec<&'a MemoryType>,
    pub globals: Vec<&'a GlobalType>,
    #[allow(dead_code)]
    pub locals: Vec<&'a ValueType>,
}

pub fn validate_module(module: &ModuleParsed) -> Result<(), ValidationError> {
    let mut context = Context::default();
    for section in module.sections.iter() {
        match section {
            Section::Type(type_section) => context.types = type_section.types.iter().collect(),
            Section::Import(_) => (), // TODO; should validate
            Section::Function(function_section) => {
                context.functions = function_section.type_indices.iter().collect();
                section::validate_function_section(function_section, &context)?
            }
            Section::Table(table_section) => context.tables = table_section.tables.iter().collect(),
            Section::Memory(memory_section) => {
                context.memories = memory_section.memories.iter().collect()
            }
            Section::Global(global_section) => {
                context.globals = global_section
                    .globals
                    .iter()
                    .map(|g| &g.global_type)
                    .collect()
            }
            Section::Export(export_section) => {
                section::validate_export_section(export_section, &context)?
            }
            Section::Start(_) => (),     // TODO; should validate
            Section::Element(_) => (),   // TODO; should validate
            Section::Code(_) => (),      // TODO; should validate
            Section::Data(_) => (),      // TODO; should validate
            Section::DataCount(_) => (), // TODO; should validate
            Section::Custom(_) => (),    // no need to validate
        }
    }
    Ok(())
}
