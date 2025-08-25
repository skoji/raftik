use super::Context;
use super::error::ValidationError;
use crate::ast::section::FunctionSection;

pub fn validate_function_section(
    function_section: &FunctionSection,
    context: &Context,
) -> Result<(), ValidationError> {
    let type_section = context
        .type_section
        .ok_or(ValidationError::NoTypeSectionInFunctionSection)?;
    let types_length = type_section.types.len();
    for (i, type_index) in function_section.type_indices.iter().enumerate() {
        if types_length as u32 <= *type_index {
            return Err(ValidationError::TypeIndexOutOfBoundsInFunctionSection(
                *type_index,
                i,
                types_length,
            ));
        }
    }
    Ok(())
}
