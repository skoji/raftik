pub mod error;
mod section;

use crate::ast::{ModuleParsed, section::Section, section::TypeSection};

#[derive(Default)]
struct Context<'a> {
    pub type_section: Option<&'a TypeSection>,
}

pub fn validate_module(module: &ModuleParsed) -> Result<(), String> {
    let mut context = Context::default();
    for section in module.sections.iter() {
        match section {
            Section::Type(type_section) => context.type_section = Some(type_section),
            Section::Function(function_section) => {
                section::validate_function_section(function_section, &context)?
            }
            _ => (),
        }
    }
    Ok(())
}
