use super::Context;
use super::error::ValidationError;
use crate::ast::section::{CollectionSection, ExportSection, FunctionSection};

pub fn validate_function_section(
    function_section: &FunctionSection,
    context: &Context,
) -> Result<(), ValidationError> {
    let type_section = context
        .type_section
        .ok_or(ValidationError::NoTypeSectionInFunctionSection)?;
    for (i, type_index) in function_section.type_indices.iter().enumerate() {
        if type_section.item(*type_index).is_none() {
            return Err(ValidationError::TypeIndexOutOfBoundsInFunctionSection(
                *type_index,
                i,
            ));
        }
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
                let function_section = context
                    .function_section
                    .ok_or(ValidationError::NoFunctionSectionInExportSection(i))?;
                if function_section.item(index).is_none() {
                    return Err(ValidationError::FunctionIndexOutOfBoundsInExportSection(
                        index, i,
                    ));
                }
            }
            ExportDesc::TableIndex(index) => {
                let table_section = context
                    .table_section
                    .ok_or(ValidationError::NoTableSectionInExportSection(i))?;
                if table_section.item(index).is_none() {
                    return Err(ValidationError::TableIndexOutOfBoundsInExportSection(
                        index, i,
                    ));
                }
            }
            ExportDesc::GlobalIndex(index) => {
                let global_section = context
                    .global_section
                    .ok_or(ValidationError::NoGlobalSectionInExportSection(i))?;
                if global_section.item(index).is_none() {
                    return Err(ValidationError::GlobalIndexOutOfBoundsInExportSection(
                        index, i,
                    ));
                }
            }
            ExportDesc::MemoryIndex(index) => {
                let memory_section = context
                    .memory_section
                    .ok_or(ValidationError::NoMemorySectionInExportSection(i))?;
                if memory_section.item(index).is_none() {
                    return Err(ValidationError::MemoryIndexOutOfBoundsInExportSection(
                        index, i,
                    ));
                }
            }
        }
    }
    Ok(())
}
