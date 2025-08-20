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
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        CodeSection, ElementSection, ExportSection, FunctionSection, GlobalSection, ImportSection,
        MemorySection, Module, Section, StartSection, TableSection, TypeSection,
        instructions::*,
        section::{
            Element, ElementItems, ElementKind, Export, ExportDesc, FunctionBody, Global, Import,
            ImportDesc, Locals, SectionID,
        },
        types::*,
    };

    fn with_wat(wat: impl AsRef<str>, test: impl Fn(Module)) {
        let wasm = wat::parse_str(wat).unwrap();
        let module = Module::from_slice(&wasm).unwrap();
        test(module)
    }

    impl Module<'_> {
        pub fn find_section(&self, id: SectionID) -> Option<&Section<'_>> {
            self.sections.iter().find(|s| s.id() == id)
        }
    }

    #[test]
    fn test_minimal_wasm() {
        with_wat("(module)", |module| assert_eq!(module, Module::default()));
    }

    #[test]
    fn test_wasm_with_type_section() {
        with_wat(
            "(module (type (func (param i32 i32) (result i64))))",
            |module| {
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
                )
            },
        );
    }

    #[test]
    fn test_wasm_with_import_section() {
        with_wat(
            "(module (import \"console\" \"log\" (func $log (param i32))))",
            |module| {
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
                )
            },
        );
    }
    #[test]
    fn test_wasm_with_function_section() {
        with_wat(
            "(module (func (param $l i32) (result i32) local.get $l))",
            |module| {
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
                        type_indices: vec![0]
                    })
                );
            },
        );
    }

    #[test]
    fn test_wasm_with_table_section() {
        with_wat("(module (table 1 10 funcref))", |module| {
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
        });
    }

    #[test]
    fn test_wasm_with_memory_section() {
        with_wat("(module (memory 100))", |module| {
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
            );
        });
    }

    #[test]
    fn test_wasm_with_global_section() {
        with_wat("(module (global i32 (i32.const 32)))", |module| {
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
            );
        });
    }

    #[test]
    fn test_wasm_with_export_section() {
        with_wat(
            "(module (func (export \"the_answer\") (result i32) i32.const 42))",
            |module| {
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
                );
            },
        );
    }

    #[test]
    fn test_wasm_with_start_section() {
        with_wat("(module (func) (start 0))", |module| {
            assert_eq!(module.sections.len(), 4); // Type, Function, Start, Code
            assert_eq!(module.sections[0].id(), SectionID::Type);
            assert_eq!(module.sections[1].id(), SectionID::Function);
            assert_eq!(
                module.sections[2],
                Section::Start(StartSection {
                    start_function_index: 0
                })
            );
        });
    }
    #[test]
    fn test_wasm_with_element_section_0() {
        with_wat(
            "(module (table 1 funcref) (func $f0) (elem (i32.const 0) func $f0))",
            |module| {
                let section = module.find_section(SectionID::Element).unwrap();
                assert_eq!(
                    *section,
                    Section::Element(ElementSection {
                        elements: vec![Element {
                            kind: ElementKind::Active {
                                table_index: None,
                                offset_expression: Expression {
                                    instructions: &[0x41, 0x00][..]
                                }
                            },
                            items: ElementItems::Functions(vec![0])
                        }],
                    })
                );
            },
        )
    }

    #[test]
    fn test_wasm_with_element_section_1() {
        with_wat("(module (func $f0) (elem func $f0))", |module| {
            let section = module.find_section(SectionID::Element).unwrap();
            assert_eq!(
                *section,
                Section::Element(ElementSection {
                    elements: vec![Element {
                        kind: ElementKind::Passive,
                        items: ElementItems::Functions(vec![0])
                    }],
                })
            );
        });
    }

    #[test]
    fn test_wasm_with_element_section_2() {
        with_wat(
            "(module  (table $t0 1 funcref) (table $t1 1 funcref) (func $f0) (elem 1 (i32.const 0) func $f0))",
            |module| {
                let section = module.find_section(SectionID::Element).unwrap();
                assert_eq!(
                    *section,
                    Section::Element(ElementSection {
                        elements: vec![Element {
                            kind: ElementKind::Active {
                                table_index: Some(1),
                                offset_expression: Expression {
                                    instructions: &[0x41, 0x00][..]
                                }
                            },
                            items: ElementItems::Functions(vec![0])
                        }],
                    })
                );
            },
        );
    }

    #[test]
    fn test_wasm_with_element_section_3() {
        with_wat("(module (func $f0) (elem declare func $f0))", |module| {
            let section = module.find_section(SectionID::Element).unwrap();
            assert_eq!(
                *section,
                Section::Element(ElementSection {
                    elements: vec![Element {
                        kind: ElementKind::Declarative,
                        items: ElementItems::Functions(vec![0])
                    }],
                })
            );
        });
    }

    #[test]
    fn test_wasm_with_element_section_4() {
        with_wat(
            "(module (table 1 funcref) (func $f0) (func) (elem (i32.const 0) funcref (ref.func 0)))",
            |module| {
                let section = module.find_section(SectionID::Element).unwrap();
                assert_eq!(
                    *section,
                    Section::Element(ElementSection {
                        elements: vec![Element {
                            kind: ElementKind::Active {
                                table_index: None,
                                offset_expression: Expression {
                                    instructions: &[0x41, 0x00][..]
                                }
                            },
                            items: ElementItems::Expressions(
                                ReferenceType::FuncRef,
                                vec![Expression {
                                    instructions: &[0xd2, 0][..]
                                }]
                            ),
                        }],
                    })
                );
            },
        )
    }

    #[test]
    fn test_wasm_with_element_section_5() {
        with_wat(
            "(module (func $f0) (elem funcref (ref.func 0)))",
            |module| {
                let section = module.find_section(SectionID::Element).unwrap();
                assert_eq!(
                    *section,
                    Section::Element(ElementSection {
                        elements: vec![Element {
                            kind: ElementKind::Passive,
                            items: ElementItems::Expressions(
                                ReferenceType::FuncRef,
                                vec![Expression {
                                    instructions: &[0xd2, 0][..]
                                }]
                            ),
                        }],
                    })
                );
            },
        );
    }

    #[test]
    fn test_wasm_with_element_section_6() {
        with_wat(
            "(module  (table $t0 1 funcref) (table $t1 1 funcref) (func $f0) (elem 1 (i32.const 0) funcref (ref.func 0)))",
            |module| {
                let section = module.find_section(SectionID::Element).unwrap();
                assert_eq!(
                    *section,
                    Section::Element(ElementSection {
                        elements: vec![Element {
                            kind: ElementKind::Active {
                                table_index: Some(1),
                                offset_expression: Expression {
                                    instructions: &[0x41, 0x00][..]
                                }
                            },
                            items: ElementItems::Expressions(
                                ReferenceType::FuncRef,
                                vec![Expression {
                                    instructions: &[0xd2, 0][..]
                                }]
                            ),
                        }],
                    })
                );
            },
        );
    }

    #[test]
    fn test_wasm_with_element_section_7() {
        with_wat(
            "(module (func $f0) (elem declare funcref (ref.func 0)))",
            |module| {
                let section = module.find_section(SectionID::Element).unwrap();
                assert_eq!(
                    *section,
                    Section::Element(ElementSection {
                        elements: vec![Element {
                            kind: ElementKind::Declarative,
                            items: ElementItems::Expressions(
                                ReferenceType::FuncRef,
                                vec![Expression {
                                    instructions: &[0xd2, 0][..]
                                }]
                            ),
                        }],
                    })
                );
            },
        );
    }

    #[test]
    fn test_wasm_with_code_section() {
        with_wat(
            "(module (func (param i32) (param i32) (result i32) (local f64) (local f64) local.get 0 local.get 1 i32.add))",
            |module| {
                let section = module.find_section(SectionID::Code).unwrap();
                assert_eq!(
                    *section,
                    Section::Code(CodeSection {
                        code: vec![FunctionBody {
                            locals: vec![Locals {
                                count: 2,
                                value_type: ValueType::Number(NumberType::F64)
                            }],
                            expression: Expression {
                                instructions: &[0x20, 0, 0x20, 1, 0x6a][..]
                            }
                        }]
                    })
                );
            },
        );
    }
}
