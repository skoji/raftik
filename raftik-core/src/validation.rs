pub mod error;
mod section;

use crate::ast::{
    FunctionSection, ModuleParsed,
    section::{Section, TypeSection},
};
use error::ValidationError;

#[derive(Default)]
struct Context<'a> {
    pub type_section: Option<&'a TypeSection>,
    pub function_section: Option<&'a FunctionSection>,
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
            Section::Table(_) => (),     // TODO; should validate
            Section::Memory(_) => (),    // TODO; should validate
            Section::Global(_) => (),    // TODO; should validate
            Section::Export(_) => (),    // TODO; should validate
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
