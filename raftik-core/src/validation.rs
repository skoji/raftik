pub mod error;
mod section;

use crate::ast::{
    FunctionSection, GlobalSection, MemorySection, ModuleParsed, Section, TableSection, TypeSection,
};

use error::ValidationError;

#[derive(Default)]
struct Context<'a> {
    pub type_section: Option<&'a TypeSection>,
    pub function_section: Option<&'a FunctionSection>,
    pub table_section: Option<&'a TableSection>,
    pub memory_section: Option<&'a MemorySection>,
    pub global_section: Option<&'a GlobalSection<'a>>,
}

pub fn validate_module(module: &ModuleParsed) -> Result<(), ValidationError> {
    let mut context = Context::default();
    for section in module.sections.iter() {
        match section {
            Section::Type(type_section) => context.type_section = Some(type_section), // no validation needed.
            Section::Import(_) => (), // TODO; should validate
            Section::Function(function_section) => {
                context.function_section = Some(function_section);
                section::validate_function_section(function_section, &context)?
            }
            Section::Table(table_section) => context.table_section = Some(table_section), // TODO; should validate
            Section::Memory(memory_section) => context.memory_section = Some(memory_section), // TODO; should validate
            Section::Global(global_section) => context.global_section = Some(global_section), // TODO; should validate
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
