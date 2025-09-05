use super::{Context, ItemFilter, error::ValidationError, types};
use crate::{
    ast::{
        section::{
            CodeSection, ElementSection, ExportSection, FunctionSection, GlobalSection,
            ImportSection, MemorySection, StartSection, TableSection,
        },
        types::{FunctionType, NumberType, ValueType},
    },
    validation::instruction,
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
            })
    }};
}

pub fn validate_function_section(
    function_section: &FunctionSection,
    context: &Context,
) -> Result<(), ValidationError> {
    for (i, type_index) in function_section.type_indices.iter().enumerate() {
        validate_index!(context.types, "Function", i, "Type", *type_index)?;
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
                validate_index!(context.functions, r, i, "Function", index)?;
            }
            ExportDesc::TableIndex(index) => {
                validate_index!(context.tables, r, i, "Table", index)?;
            }
            ExportDesc::GlobalIndex(index) => {
                validate_index!(context.globals, r, i, "Global", index)?;
            }
            ExportDesc::MemoryIndex(index) => {
                validate_index!(context.memories, r, i, "Memory", index)?;
            }
        }
    }
    Ok(())
}

pub fn validate_code_section<'a>(
    code_section: &'a CodeSection<'a>,
    context: &mut Context<'a>,
) -> Result<(), ValidationError> {
    let funcs_declared: Vec<_> = context
        .functions
        .internal()
        .iter()
        .map(|x| *x.t())
        .collect();
    let code_bodies = code_section.code.len();
    if funcs_declared.len() != code_bodies {
        return Err(ValidationError::CodeSectionLengthMismatch {
            funcs_declared: funcs_declared.len(),
            code_bodies,
        });
    }
    for (i, funcbody) in code_section.code.iter().enumerate() {
        let type_index = funcs_declared[i];
        let func_type = context.types[type_index as usize];

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

const MAX_TABLE_SIZE: u32 = u32::MAX;
pub fn validate_table_section(table_section: &TableSection) -> Result<(), ValidationError> {
    for (i, table) in table_section.tables.iter().enumerate() {
        if !types::validate_limits(&table.limits, MAX_TABLE_SIZE) {
            return Err(ValidationError::TableSizeError {
                section: "Table".to_string(),
                index: i,
                limits: table.limits.clone(),
                maximum: MAX_TABLE_SIZE,
            });
        }
    }
    Ok(())
}

const MAX_PAGES_SIZE: u32 = 2_u32.pow(16);
pub fn validate_memory_section(memory_section: &MemorySection) -> Result<(), ValidationError> {
    for (i, memory) in memory_section.memories.iter().enumerate() {
        if !types::validate_limits(&memory.limits, MAX_PAGES_SIZE) {
            return Err(ValidationError::MemorySizeError {
                section: "Memory".to_string(),
                index: i,
                limits: memory.limits.clone(),
                maximum: MAX_PAGES_SIZE,
            });
        }
    }
    Ok(())
}

pub fn validate_global_section(
    global_section: &GlobalSection,
    ctx: &mut Context,
) -> Result<(), ValidationError> {
    for (i, g) in global_section.globals.iter().enumerate() {
        let f = FunctionType {
            params: vec![],
            results: vec![g.global_type.val_type],
        };
        super::instruction::validate_raw_expression(
            ctx,
            &f,
            &g.expression,
            format!("at global section {}", i),
        )?;
    }
    Ok(())
}

pub fn validate_import_section(
    import_section: &ImportSection,
    ctx: &Context,
) -> Result<(), ValidationError> {
    use crate::ast::section::ImportDesc;
    for (i, im) in import_section.imports.iter().enumerate() {
        match &im.desc {
            ImportDesc::TypeIndex(index) => {
                validate_index!(ctx.types, "Import", i, "Function", *index)?;
            }
            ImportDesc::Table(table_type) => {
                if !types::validate_limits(&table_type.limits, MAX_TABLE_SIZE) {
                    return Err(ValidationError::TableSizeError {
                        section: "Import".to_string(),
                        index: i,
                        limits: table_type.limits.clone(),
                        maximum: MAX_TABLE_SIZE,
                    });
                }
            }
            ImportDesc::Memory(memory_type) => {
                if !types::validate_limits(&memory_type.limits, MAX_PAGES_SIZE) {
                    return Err(ValidationError::MemorySizeError {
                        section: "Import".to_string(),
                        index: i,
                        limits: memory_type.limits.clone(),
                        maximum: MAX_PAGES_SIZE,
                    });
                }
            }
            ImportDesc::Global(_) => (), // nothing to validate
        }
    }
    Ok(())
}

pub fn validate_start_section(
    start_section: &StartSection,
    ctx: &Context,
) -> Result<(), ValidationError> {
    let t = validate_index!(
        ctx.functions,
        "Start",
        0,
        "Function",
        start_section.start_function_index
    )?
    .t();
    let f = *validate_index!(ctx.types, "Start", 0, "Types", *t)?;

    if !f.params.is_empty() || !f.results.is_empty() {
        Err(ValidationError::StartFuncInvalid {
            functype: f.clone(),
        })
    } else {
        Ok(())
    }
}

pub fn validate_element_section(
    element_section: &ElementSection,
    ctx: &mut Context,
) -> Result<(), ValidationError> {
    for (i, e) in element_section.elements.iter().enumerate() {
        match e.kind {
            crate::ast::section::ElementKind::Active {
                table_index,
                ref offset_expression,
            } => {
                let table_index = table_index.unwrap_or(0);
                validate_index!(ctx.tables, "Element", i, "Table", table_index)?;
                let f = FunctionType {
                    params: vec![],
                    results: vec![ValueType::Number(NumberType::I32)],
                };
                instruction::validate_raw_expression(
                    ctx,
                    &f,
                    offset_expression,
                    format!("at element section #{}", i),
                )?;
            }
            crate::ast::section::ElementKind::Declarative => (),
            crate::ast::section::ElementKind::Passive => (),
        }
        match &e.items {
            crate::ast::section::ElementItems::Functions(items) => {
                for index in items.iter() {
                    // TODO; should include the count of item in error message
                    validate_index!(ctx.functions, "Element-Items", i, "Function", *index)?;
                }
            }
            crate::ast::section::ElementItems::Expressions(reference_type, raw_expressions) => {
                let f = FunctionType {
                    params: vec![],
                    results: vec![ValueType::Reference(*reference_type)],
                };
                for (j, e) in raw_expressions.iter().enumerate() {
                    instruction::validate_raw_expression(
                        ctx,
                        &f,
                        e,
                        format!("at element section #{}, item #{}", i, j),
                    )?;
                }
            }
        }
    }
    Ok(())
}
