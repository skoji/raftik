use super::RawSection;
use super::integer::parse_varuint32;
use super::types::parse_function_type;
use crate::ast::section::TypeSection;
use crate::binary::raw_module::SectionID;
use nom::Parser;
use nom::multi::length_count;

impl<'a> TryFrom<RawSection<'a>> for TypeSection {
    type Error = String;

    fn try_from(raw: RawSection<'a>) -> Result<Self, Self::Error> {
        if raw.header.id != SectionID::Type {
            return Err("RawSection is not a TypeSection".to_string());
        }
        let (_, types) = length_count(parse_varuint32, parse_function_type)
            .parse(raw.payload)
            .map_err(|e| format!("Failed to parse TypeSection: {}", e))?;
        Ok(TypeSection { types })
    }
}
