pub mod error;
mod instruction;
mod section;
mod types;

use error::ValidationError;

use crate::ast::{
    ModuleParsed, Section,
    types::{FunctionType, GlobalType, MemoryType, TableType, ValueType},
};

#[allow(dead_code)]
#[derive(Debug)]
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
    fn internal(&self) -> Vec<T>;
    fn imported(&self) -> Vec<T>;
}

impl<T: Clone> ItemFilter<T> for Vec<ItemDesc<T>> {
    fn internal(&self) -> Vec<T> {
        self.iter()
            .filter(|x| match x {
                ItemDesc::Internal { .. } => true,
                ItemDesc::Imported { .. } => false,
            })
            .map(|x| x.t().clone())
            .collect()
    }

    fn imported(&self) -> Vec<T> {
        self.iter()
            .filter(|x| match x {
                ItemDesc::Internal { .. } => false,
                ItemDesc::Imported { .. } => true,
            })
            .map(|x| x.t().clone())
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
                for g in global_section
                    .globals
                    .iter()
                    .map(|g| ItemDesc::Internal { t: &g.global_type })
                {
                    context.globals.push(g);
                }
            }
            Section::Export(_) => (),
            Section::Start(_) => (),
            Section::Element(_) => (),
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
            Section::Global(_) => (), // TODO; should validate
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

    impl<'a> ModuleParsed<'a> {
        pub fn sec_by_id(&self, id: SectionID) -> Option<&Section<'a>> {
            self.sections.iter().find(|s| s.id() == id)
        }
        pub fn sec_by_id_mut(&mut self, id: SectionID) -> Option<&mut Section<'a>> {
            self.sections.iter_mut().find(|s| s.id() == id)
        }
    }

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
}
