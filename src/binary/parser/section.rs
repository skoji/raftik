use super::RawSection;
use super::integer::parse_varuint32;
use super::name::parse_name;
use super::types::{parse_function_type, parse_global_type, parse_limits, parse_table_type};
use crate::ast::section::{Import, ImportDesc, ImportSection, TypeSection};
use crate::binary::raw_module::SectionID;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
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

impl<'a> TryFrom<RawSection<'a>> for ImportSection {
    type Error = String;

    fn try_from(raw: RawSection<'a>) -> Result<Self, Self::Error> {
        if raw.header.id != SectionID::Import {
            return Err("RawSection is not an ImportSection".to_string());
        }
        let (_, imports) = length_count(parse_varuint32, parse_import)
            .parse(raw.payload)
            .map_err(|e| format!("Failed to parse ImportSection: {}", e))?;
        Ok(ImportSection { imports })
    }
}

fn parse_import(input: &[u8]) -> nom::IResult<&[u8], Import> {
    map(
        (parse_name, parse_name, parse_import_desc),
        |(module, name, desc)| Import { module, name, desc },
    )
    .parse(input)
}

fn parse_import_desc(input: &[u8]) -> nom::IResult<&[u8], ImportDesc> {
    alt((
        map((tag(&[0x00][..]), parse_varuint32), |(_, type_index)| {
            ImportDesc::TypeIndex(type_index)
        }),
        map((tag(&[0x01][..]), parse_table_type), |(_, table_type)| {
            ImportDesc::Table(table_type)
        }),
        map((tag(&[0x02][..]), parse_limits), |(_, limits)| {
            ImportDesc::Memory(limits)
        }),
        map((tag(&[0x03][..]), parse_global_type), |(_, global_type)| {
            ImportDesc::Global(global_type)
        }),
    ))
    .parse(input)
}
