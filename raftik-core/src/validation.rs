pub mod error;
mod instruction;
mod section;

use error::ValidationError;

use crate::ast::{
    ModuleParsed, Section,
    types::{FunctionType, GlobalType, MemoryType, TableType, ValueType},
};

#[derive(Default, Debug)]
struct Context<'a> {
    pub types: Vec<&'a FunctionType>,
    pub functions: Vec<u32>,
    pub tables: Vec<&'a TableType>,
    pub memories: Vec<&'a MemoryType>,
    pub globals: Vec<&'a GlobalType>,
    pub locals: Vec<ValueType>,
}

fn initialize_context<'a>(module: &'a ModuleParsed<'a>) -> Result<Context<'a>, ValidationError> {
    let mut context = Context::default();
    for section in module.sections.iter() {
        match section {
            Section::Type(type_section) => context.types = type_section.types.iter().collect(),
            Section::Import(_) => (),
            Section::Function(function_section) => {
                context.functions = function_section.type_indices.iter().cloned().collect();
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
            Section::Export(_) => (),
            Section::Start(_) => (),
            Section::Element(_) => (),
            Section::Code(_) => (),
            Section::Data(_) => (),
            Section::DataCount(_) => (),
            Section::Custom(_) => (),
        }
    }
    Ok(context)
}

pub fn validate_module(module: &ModuleParsed) -> Result<(), ValidationError> {
    #[allow(unused_mut)]
    let mut context = initialize_context(module)?;
    for section in module.sections.iter() {
        match section {
            Section::Type(_) => (),   // no need to validate
            Section::Import(_) => (), // TODO; should validate
            Section::Function(function_section) => {
                section::validate_function_section(function_section, &context)?
            }
            Section::Table(_) => (),  // TODO; should validate
            Section::Memory(_) => (), // TODO; should validate
            Section::Global(_) => (), // TODO; should validate
            Section::Export(export_section) => {
                section::validate_export_section(export_section, &context)?
            }
            Section::Start(_) => (),   // TODO; should validate
            Section::Element(_) => (), // TODO; should validate
            Section::Code(code_section) => {
                section::validate_code_section(code_section, &mut context)?
            }
            Section::Data(_) => (),      // TODO; should validate
            Section::DataCount(_) => (), // TODO; should validate
            Section::Custom(_) => (),    // no need to validate
        }
    }
    Ok(())
}
