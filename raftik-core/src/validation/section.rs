use super::Context;
use super::error::ValidationError;
use crate::ast::section::{CollectionSection, ExportSection, FunctionSection};

macro_rules! validate_index {
    ($referring_section_name:expr, $section_field:expr, $section_name:expr, $index:expr, $i:expr) => {{
        let section = $section_field.ok_or(ValidationError::ReferingSectionNotFound(
            $referring_section_name,
            $section_name,
        ))?;
        if section.item($index).is_none() {
            return Err(ValidationError::IndexOutOfBoundsInSection(
                $referring_section_name,
                $section_name,
                $index,
                $i,
            ));
        }
    }};
}

pub fn validate_function_section(
    function_section: &FunctionSection,
    context: &Context,
) -> Result<(), ValidationError> {
    for (i, type_index) in function_section.type_indices.iter().enumerate() {
        validate_index!("Function", context.type_section, "Type", *type_index, i);
    }
    Ok(())
}

pub fn validate_export_section(
    export_section: &ExportSection,
    context: &Context,
) -> Result<(), ValidationError> {
    use crate::ast::section::ExportDesc;
    for (i, export) in export_section.exports.iter().enumerate() {
        match export.desc {
            ExportDesc::FunctionIndex(index) => {
                validate_index!("Export", context.function_section, "Function", index, i)
            }
            ExportDesc::TableIndex(index) => {
                validate_index!("Export", context.table_section, "Table", index, i);
            }
            ExportDesc::GlobalIndex(index) => {
                validate_index!("Export", context.global_section, "Global", index, i);
            }
            ExportDesc::MemoryIndex(index) => {
                validate_index!("Export", context.memory_section, "Memory", index, i);
            }
        }
    }
    Ok(())
}
