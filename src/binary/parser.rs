mod integer;
mod leb128;
mod module;
mod name;
mod section_parser_trait;
mod types;

use module::parse_module;

use crate::ast::Module;

impl<'a> TryFrom<&'a [u8]> for Module<'a> {
    type Error = String;

    fn try_from(input: &'a [u8]) -> Result<Self, Self::Error> {
        let (_, module) =
            parse_module(input).map_err(|e| format!("Failed to parse module: {:?}", e))?;
        Ok(module)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        FunctionSection, ImportSection, Module, Section, TypeSection,
        section::{Import, ImportDesc},
        types::*,
    };
    #[test]
    fn test_minimal_wasm() {
        let wasm = wat::parse_str("(module)").unwrap();
        let module = Module::try_from(wasm.as_ref()).unwrap();
        assert_eq!(module, Module::default());
    }

    #[test]
    fn test_wasm_with_type_section() {
        let wasm = wat::parse_str("(module (type (func (param i32 i32) (result i64))))").unwrap();
        let module = Module::try_from(wasm.as_ref()).unwrap();
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
        let module = Module::try_from(wasm.as_ref()).unwrap();
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
                    desc: ImportDesc::TypeIndex(TypeIndex { index: 0 })
                }]
            })
        );
    }
    #[test]
    fn test_wasm_with_function_section() {
        let wasm =
            wat::parse_str("(module (func (param $l i32) (result i32) local.get $l))").unwrap();
        let module = Module::try_from(wasm.as_ref()).unwrap();
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
                type_indexes: vec![TypeIndex { index: 0 }]
            })
        );
    }
}
