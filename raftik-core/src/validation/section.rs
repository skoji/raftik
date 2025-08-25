use super::Context;
use crate::ast::section::FunctionSection;

pub fn validate_function_section(
    function_section: &FunctionSection,
    context: &Context,
) -> Result<(), String> {
    let type_section = context
        .type_section
        .ok_or("no type index section".to_string())?;
    let types_length = type_section.types.len();
    for (_, type_index) in function_section.type_indices.iter().enumerate() {
        if types_length as u32 <= *type_index {
            return Err("type index does not exist".to_string());
        }
    }
    Ok(())
}
