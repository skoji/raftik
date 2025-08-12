mod integer;
mod leb128;
mod raw_module;
mod section;

use super::raw_module::{RawSection, SectionID};
use crate::ast::Module;
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
                    assign_once(&mut module.type_section, rs.try_into()?)?;
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

fn assign_once<T>(slot: &mut Option<T>, val: T) -> Result<(), String> {
    if slot.is_some() {
        return Err("Duplicate Section".to_string());
    }
    *slot = Some(val);
    Ok(())
}
