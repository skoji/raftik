mod integer;
mod leb128;
mod raw_module;
mod section;
mod types;

use super::raw_module::{RawSection, SectionID};
use crate::ast::{Module, Section};
use raw_module::parse_raw_module;

impl TryFrom<&[u8]> for Module {
    type Error = String;

    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let (remaining, raw) =
            parse_raw_module(input).map_err(|e| format!("Failed to parse raw module: {}", e))?;

        if !remaining.is_empty() {
            return Err("Extra data after module".to_string());
        }

        // check id order
        let mut last_id = 0u8;
        for rs in &raw.sections {
            if rs.header.id == SectionID::Custom {
                continue; // Custom sections are not checked for ID order
            }
            if rs.header.id as u8 <= last_id {
                return Err(format!(
                    "Section IDs must be in ascending order, found {} after {}",
                    rs.header.id as u8, last_id
                ));
            }
            last_id = rs.header.id as u8;
        }

        let mut module = Module::default();

        for rs in raw.sections {
            match rs.header.id {
                SectionID::Type => {
                    let type_section = rs.try_into()?;
                    module.sections.push(Section::Type(type_section));
                }
                _ => {
                    // Handle other sections as needed
                    // For now, we will just ignore them
                    // You can implement further logic to handle other sections
                }
            }
        }
        Ok(module)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::types::*;
    use crate::ast::{Module, Section, TypeSection};
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
}
