use super::Context;
use super::error::ValidationError;
use crate::ast::section::{ExportSection, FunctionSection};

macro_rules! validate_index {
    ($field: expr, $refering: expr, $refering_index: expr, $refered: expr, $refered_index: expr) => {{
        $field
            .get($refered_index as usize)
            .ok_or(ValidationError::IndexOutOfBoundsIn {
                refering: $refering,
                refering_index: $refering_index,
                refered: $refered,
                refered_index: $refered_index,
            })?;
    }};
}

pub fn validate_function_section(
    function_section: &FunctionSection,
    context: &Context,
) -> Result<(), ValidationError> {
    for (i, type_index) in function_section.type_indices.iter().enumerate() {
        validate_index!(context.types, "Function", i, "Type", *type_index);
    }
    Ok(())
}

pub fn validate_export_section(
    export_section: &ExportSection,
    context: &Context,
) -> Result<(), ValidationError> {
    use crate::ast::section::ExportDesc;
    let r = "Export";
    for (i, export) in export_section.exports.iter().enumerate() {
        match export.desc {
            ExportDesc::FunctionIndex(index) => {
                validate_index!(context.functions, r, i, "Function", index)
            }
            ExportDesc::TableIndex(index) => {
                validate_index!(context.tables, r, i, "Table", index);
            }
            ExportDesc::GlobalIndex(index) => {
                validate_index!(context.globals, r, i, "Global", index);
            }
            ExportDesc::MemoryIndex(index) => {
                validate_index!(context.memories, r, i, "Memory", index);
            }
        }
    }
    Ok(())
}
