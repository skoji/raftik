pub mod error;
mod instruction;
mod section;
mod types;

use std::collections::HashSet;

use error::ValidationError;

use crate::ast::{
    ModuleParsed, Section,
    section::ExportDesc,
    types::{FunctionType, GlobalType, MemoryType, ReferenceType, TableType, ValueType},
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum ItemDesc<T: Clone> {
    Internal { t: T },
    Imported { module: String, name: String, t: T },
}

impl<T: Clone> ItemDesc<T> {
    fn t(&self) -> &T {
        match self {
            ItemDesc::Internal { t } => t,
            ItemDesc::Imported { t, .. } => t,
        }
    }
}

trait ItemFilter<T: Clone> {
    fn internal(&self) -> Vec<ItemDesc<T>>;
    fn imported(&self) -> Vec<ItemDesc<T>>;
}

impl<T: Clone> ItemFilter<T> for Vec<ItemDesc<T>> {
    fn internal(&self) -> Vec<ItemDesc<T>> {
        self.iter()
            .filter(|x| match x {
                ItemDesc::Internal { .. } => true,
                ItemDesc::Imported { .. } => false,
            })
            .cloned()
            .collect()
    }
    fn imported(&self) -> Vec<ItemDesc<T>> {
        self.iter()
            .filter(|x| match x {
                ItemDesc::Internal { .. } => false,
                ItemDesc::Imported { .. } => true,
            })
            .cloned()
            .collect()
    }
}

#[derive(Default, Debug)]
struct Context<'a> {
    pub types: Vec<&'a FunctionType>,
    pub functions: Vec<ItemDesc<u32>>,
    pub tables: Vec<&'a TableType>,
    pub memories: Vec<&'a MemoryType>,
    pub globals: Vec<ItemDesc<&'a GlobalType>>,
    pub locals: Vec<ValueType>,
    pub refs: HashSet<u32>,
    pub instructions_should_be_constant: bool,
}

impl<'a> Context<'a> {
    pub fn prime(&mut self) -> Self {
        Context {
            types: self.types.clone(),
            functions: self.functions.clone(),
            tables: self.tables.clone(),
            memories: self.memories.clone(),
            globals: self.globals.imported(),
            locals: self.locals.clone(),
            refs: self.refs.clone(),
            instructions_should_be_constant: self.instructions_should_be_constant,
        }
    }
}

fn initialize_context<'a>(module: &'a ModuleParsed<'a>) -> Result<Context<'a>, ValidationError> {
    let mut context = Context::default();
    for section in module.sections.iter() {
        match section {
            Section::Type(type_section) => context.types = type_section.types.iter().collect(),
            Section::Import(import_section) => {
                for i in &import_section.imports {
                    match &i.desc {
                        crate::ast::section::ImportDesc::TypeIndex(t) => {
                            context.functions.push(ItemDesc::Imported {
                                name: i.name.clone(),
                                module: i.module.clone(),
                                t: *t,
                            });
                        }
                        crate::ast::section::ImportDesc::Table(table_type) => {
                            context.tables.push(table_type);
                        }
                        crate::ast::section::ImportDesc::Memory(memory_type) => {
                            context.memories.push(memory_type);
                        }
                        crate::ast::section::ImportDesc::Global(t) => {
                            context.globals.push(ItemDesc::Imported {
                                name: i.name.clone(),
                                module: i.module.clone(),
                                t,
                            });
                        }
                    }
                }
            }
            Section::Function(function_section) => {
                for t in function_section.type_indices.iter() {
                    context.functions.push(ItemDesc::Internal { t: *t });
                }
            }
            Section::Table(table_section) => {
                for t in table_section.tables.iter() {
                    context.tables.push(t);
                }
            }
            Section::Memory(memory_section) => {
                for m in memory_section.memories.iter() {
                    context.memories.push(m);
                }
            }
            Section::Global(global_section) => {
                for (i, g) in global_section.globals.iter().enumerate() {
                    if let ValueType::Reference(ReferenceType::FuncRef) = g.global_type.val_type {
                        instruction::collect_funcref_in_expression(
                            &g.expression,
                            &mut context.refs,
                            format!("in global section {}", i),
                        )?;
                    }
                    let g = ItemDesc::Internal { t: &g.global_type };
                    context.globals.push(g);
                }
            }
            Section::Export(export_section) => {
                for e in export_section.exports.iter() {
                    if let ExportDesc::FunctionIndex(i) = e.desc {
                        context.refs.insert(i);
                    }
                }
            }
            Section::Start(_) => (),
            Section::Element(element_section) => {
                for (i, e) in element_section.elements.iter().enumerate() {
                    match &e.items {
                        crate::ast::section::ElementItems::Functions(items) => {
                            for i in items {
                                context.refs.insert(*i);
                            }
                        }
                        crate::ast::section::ElementItems::Expressions(
                            reference_type,
                            raw_expressions,
                        ) => {
                            if *reference_type == ReferenceType::FuncRef {
                                for (j, exp) in raw_expressions.iter().enumerate() {
                                    instruction::collect_funcref_in_expression(
                                        exp,
                                        &mut context.refs,
                                        format!(
                                            "in element section item #{}, expression #{}",
                                            i, j
                                        ),
                                    )?;
                                }
                            }
                        }
                    }
                }
            }
            Section::Code(_) => (),
            Section::Data(_) => (),
            Section::DataCount(_) => (),
            Section::Custom(_) => (),
        }
    }
    Ok(context)
}

pub fn validate_module(module: &ModuleParsed) -> Result<(), ValidationError> {
    let mut context = initialize_context(module)?;
    for section in module.sections.iter() {
        match section {
            Section::Type(_) => (),   // no need to validate
            Section::Import(_) => (), // TODO; should validate
            Section::Function(function_section) => {
                section::validate_function_section(function_section, &context)?
            }
            Section::Table(table_section) => section::validate_table_section(table_section)?,
            Section::Memory(memory_section) => section::validate_memory_section(memory_section)?,
            Section::Global(global_section) => {
                let mut c_prime = context.prime();
                c_prime.instructions_should_be_constant = true;
                section::validate_global_section(global_section, &mut c_prime)?
            }
            Section::Export(export_section) => {
                section::validate_export_section(export_section, &context)?
            }
            Section::Start(_) => (),   // TODO; should validate
            Section::Element(_) => (), // TODO; should validate
            Section::Code(code_section) => {
                section::validate_code_section(code_section, &mut context)?
            }
            Section::Data(_) => (),      // TODO; should validate
            Section::DataCount(_) => (), // TODO; should validate
            Section::Custom(_) => (),    // no need to validate
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{ModuleParsed, section::SectionID, types::*},
        validation::error::VInstError,
    };

    fn with_wat(wat: impl AsRef<str>, test: impl Fn(ModuleParsed)) {
        let wasm = wat::parse_str(wat).unwrap();
        let module = ModuleParsed::from_slice(&wasm).unwrap();
        test(module)
    }

    #[test]
    fn test_minimal_wasm() {
        with_wat("(module)", |module| {
            assert!(validate_module(&module).is_ok());
        });
    }

    #[test]
    fn test_add_module() {
        with_wat(
            "
(module
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.add)
  (export \"add\" (func $add))
)
",
            |module| assert!(validate_module(&module).is_ok()),
        );
    }

    #[test]
    fn test_invalid_add_module() {
        with_wat(
            "
(module
  (func $add (param $lhs i32) (param $rhs i64) (result i32)
    local.get $lhs
    local.get $rhs
    i32.add)
  (export \"add\" (func $add))
)
",
            |module| match validate_module(&module) {
                Ok(_) => unreachable!("should produce error"),
                Err(e) => match e {
                    ValidationError::InstructionValidationError { error, .. } => {
                        if let VInstError::PopValueTypeMismatch { expected, actual } = error {
                            assert_eq!(ValueType::Number(NumberType::I32), expected);
                            assert_eq!(ValueType::Number(NumberType::I64), actual);
                        } else {
                            unreachable!("unexpected VInstError: {:?}", error);
                        }
                    }
                    _ => unreachable!("unexpected error type: {:?}", e),
                },
            },
        );
    }

    #[test]
    fn test_table_module() {
        with_wat("(module (table 1 10 funcref))", |module| {
            assert!(validate_module(&module).is_ok());
        });
    }

    #[test]
    fn test_invalid_table_module() {
        with_wat("(module (table 1 10 funcref))", |mut module| {
            let section: &mut Section<'_> = module.sec_by_id_mut(SectionID::Table).unwrap();
            let Section::Table(table_section) = section else {
                unreachable!("")
            };
            table_section.tables[0].limits.min = 12;

            match validate_module(&module) {
                Ok(_) => unreachable!("should produce error"),
                Err(e) => match e {
                    ValidationError::TableSizeError { .. } => (),
                    _ => unreachable!("unexpected error type: {:?}", e),
                },
            };
        });
    }

    #[test]
    fn test_memory_module() {
        with_wat("(module (memory 10 100))", |module| {
            assert!(validate_module(&module).is_ok());
        });
    }

    #[test]
    fn test_invalid_memory_module() {
        with_wat("(module (memory 10 100))", |mut module| {
            let section: &mut Section<'_> = module.sec_by_id_mut(SectionID::Memory).unwrap();
            let Section::Memory(memory_section) = section else {
                unreachable!("")
            };
            memory_section.memories[0].limits.max = Some(1);
            match validate_module(&module) {
                Ok(_) => unreachable!("should produce error"),
                Err(e) => match e {
                    ValidationError::MemorySizeError { .. } => (),
                    _ => unreachable!("unexpected error type: {:?}", e),
                },
            }
        });
    }

    #[test]
    fn test_global_section_with_function_ref() {
        with_wat(
            "(module (global funcref ref.func 0) (func (param i32) (param i32) (result i32) local.get 0 local.get 1 i32.add))",
            |module| {
                assert!(validate_module(&module).is_ok());
            },
        );
    }

    #[test]
    fn test_global_section_with_invalid_static() {
        with_wat(
            "(module (global i32 i32.const 0 i32.const 1 i32.add) (func (param i32) (param i32) (result i32) local.get 0 local.get 1 i32.add))",
            |module| {
                let r = validate_module(&module);
                if let Err(ValidationError::InstructionValidationError { error: e, .. }) = r {
                    assert!(matches!(e, VInstError::OpcodeShouldBeConstant(_)));
                } else {
                    unreachable!("result not expected: {:#?}", r);
                }
            },
        );
    }
}
