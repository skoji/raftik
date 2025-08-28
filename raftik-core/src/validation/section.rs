use super::{Context, error::ValidationError};
use crate::ast::{
    CodeSection,
    section::{ExportSection, FunctionSection},
};

macro_rules! validate_index {
    ($field: expr, $referring: expr, $referring_index: expr, $referred: expr, $referred_index: expr) => {{
        $field
            .get($referred_index as usize)
            .ok_or(ValidationError::IndexOutOfBoundsIn {
                referring: $referring,
                referring_index: $referring_index,
                referred: $referred,
                referred_index: $referred_index,
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

pub fn validate_code_section<'a>(
    code_section: &'a CodeSection<'a>,
    context: &mut Context<'a>,
) -> Result<(), ValidationError> {
    let funcs_declared = context.functions.len();
    let code_bodies = code_section.code.len();
    if funcs_declared != code_bodies {
        return Err(ValidationError::CodeSectionLengthMismatch {
            funcs_declared,
            code_bodies,
        });
    }
    for (i, funcbody) in code_section.code.iter().enumerate() {
        let type_index = context.functions[i];
        let func_type = context.types[*type_index as usize];

        context.locals.clear();
        for param in func_type.params.iter() {
            context.locals.push(*param);
        }
        for local in funcbody.locals.iter() {
            for _ in 0..local.count {
                context.locals.push(local.value_type)
            }
        }
        super::instruction::validate_raw_expression(
            context,
            func_type,
            &funcbody.expression,
            format!("at code section #{}", i),
        )?;
    }
    context.locals.clear();
    Ok(())
}
