mod instructions;
mod integer;
mod leb128;
mod module;
mod name;
mod section_parser_trait;
mod types;

use module::parse_module;

use crate::ast::Module;

impl<'a> Module<'a> {
    pub fn from_slice(input: &'a [u8]) -> Result<Self, String> {
        let (_, module) =
            parse_module(input).map_err(|e| format!("Failed to parse module: {:?}", e))?;
        // check section order
        let mut last_id = 0u8;
        for s in &module.sections {
            let id = s.id() as u8;
            if id == 0 {
                continue;
            }
            if id <= last_id {
                return Err(format!(
                    "Sections are not in the correct order: {:?} < {:?}",
                    s.id(),
                    last_id
                ));
            }
            last_id = id;
        }
        Ok(module)
    }

    pub fn from_bytes(bytes: &'a Vec<u8>) -> Result<Self, String> {
        Module::from_slice(bytes.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        ExportSection, FunctionSection, GlobalSection, ImportSection, MemorySection, Module,
        Section, StartSection, TableSection, TypeSection,
        instructions::*,
        section::{Export, ExportDesc, Global, Import, ImportDesc, SectionID},
        types::*,
    };
    #[test]
    fn test_minimal_wasm() {
        let wasm = wat::parse_str("(module)").unwrap();
        let module = Module::from_bytes(wasm.as_ref()).unwrap();
        assert_eq!(module, Module::default());
    }

    #[test]
    fn test_wasm_with_type_section() {
        let wasm = wat::parse_str("(module (type (func (param i32 i32) (result i64))))").unwrap();
        let module = Module::from_bytes(wasm.as_ref()).unwrap();
        assert_eq!(
            module,
            Module {
                sections: vec![Section::Type(TypeSection {
                    types: vec![FunctionType {
                        params: vec![
                            ValueType::Number(NumberType::I32),
                            ValueType::Number(NumberType::I32)
                        ],
                        results: vec![ValueType::Number(NumberType::I64)]
                    }]
                })]
            }
        );
    }

    #[test]
    fn test_wasm_with_import_section() {
        let wasm = wat::parse_str("(module (import \"console\" \"log\" (func $log (param i32))))")
            .unwrap();
        let module = Module::from_bytes(wasm.as_ref()).unwrap();
        // type section and import section exists.
        assert_eq!(module.sections.len(), 3);
        assert_eq!(
            module.sections[0],
            Section::Type(TypeSection {
                types: vec![FunctionType {
                    params: vec![ValueType::Number(NumberType::I32)],
                    results: vec![]
                }]
            })
        );
        assert_eq!(
            module.sections[1],
            Section::Import(ImportSection {
                imports: vec![Import {
                    module: "console".to_string(),
                    name: "log".to_string(),
                    desc: ImportDesc::TypeIndex(0)
                }]
            })
        );
    }
    #[test]
    fn test_wasm_with_function_section() {
        let wasm =
            wat::parse_str("(module (func (param $l i32) (result i32) local.get $l))").unwrap();
        let module = Module::from_bytes(wasm.as_ref()).unwrap();
        assert_eq!(module.sections.len(), 4);
        assert_eq!(
            module.sections[0],
            Section::Type(TypeSection {
                types: vec![FunctionType {
                    params: vec![ValueType::Number(NumberType::I32)],
                    results: vec![ValueType::Number(NumberType::I32)]
                }]
            })
        );
        assert_eq!(
            module.sections[1],
            Section::Function(FunctionSection {
                type_indexes: vec![0]
            })
        );
    }

    #[test]
    fn test_wasm_with_table_section() {
        let wasm = wat::parse_str("(module (table 1 10 funcref))").unwrap();
        let module = Module::from_bytes(wasm.as_ref()).unwrap();
        assert_eq!(module.sections.len(), 1);
        assert_eq!(
            module.sections[0],
            Section::Table(TableSection {
                tables: vec![TableType {
                    ref_type: ReferenceType::FuncRef,
                    limits: Limits {
                        min: 1,
                        max: Some(10)
                    }
                }]
            })
        );
    }

    #[test]
    fn test_wasm_with_memory_section() {
        let wasm = wat::parse_str("(module (memory 100))").unwrap();
        let module = Module::from_bytes(wasm.as_ref()).unwrap();
        assert_eq!(module.sections.len(), 1);
        assert_eq!(
            module.sections[0],
            Section::Memory(MemorySection {
                memories: vec![MemoryType {
                    limits: Limits {
                        min: 100,
                        max: None,
                    }
                }]
            })
        )
    }

    #[test]
    fn test_wasm_with_global_section() {
        let wasm = wat::parse_str("(module (global i32 (i32.const 32)))").unwrap();
        let module = Module::from_bytes(wasm.as_ref()).unwrap();
        assert_eq!(module.sections.len(), 1);
        assert_eq!(
            module.sections[0],
            Section::Global(GlobalSection {
                globals: vec![Global {
                    global_type: GlobalType {
                        val_type: ValueType::Number(NumberType::I32),
                        mutability: Mutability::Const,
                    },
                    expression: Expression {
                        instructions: &[0x41, 0x20][..]
                    }
                }]
            })
        )
    }

    #[test]
    fn test_wasm_with_export_section() {
        let wasm =
            wat::parse_str("(module (func (export \"the_answer\") (result i32) i32.const 42))")
                .unwrap();
        let module = Module::from_bytes(wasm.as_ref()).unwrap();
        assert_eq!(module.sections.len(), 4); // Type, Function, Export, Code
        assert_eq!(module.sections[0].id(), SectionID::Type);
        assert_eq!(module.sections[1].id(), SectionID::Function);
        assert_eq!(
            module.sections[2],
            Section::Export(ExportSection {
                exports: vec![Export {
                    name: "the_answer".to_string(),
                    desc: ExportDesc::FunctionIndex(0),
                }]
            })
        )
    }

    #[test]
    fn test_wasm_with_start_section() {
        let wasm = wat::parse_str("(module (func) (start 0))").unwrap();
        let module = Module::from_bytes(wasm.as_ref()).unwrap();
        assert_eq!(module.sections.len(), 4); // Type, Function, Start, Code
        assert_eq!(module.sections[0].id(), SectionID::Type);
        assert_eq!(module.sections[1].id(), SectionID::Function);
        assert_eq!(
            module.sections[2],
            Section::Start(StartSection {
                start_function_index: 0
            })
        );
    }
}
