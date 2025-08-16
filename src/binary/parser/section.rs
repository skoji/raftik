use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, combinator::map, multi::length_count,
};

use super::{
    RawSection,
    integer::parse_varuint32,
    name::parse_name,
    types::{
        parse_function_type, parse_global_type, parse_memory_type, parse_table_type,
        parse_type_index,
    },
};
use crate::{
    ast::section::{FunctionSection, Import, ImportDesc, ImportSection, TypeSection},
    binary::raw_module::SectionID,
};

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

fn parse_import(input: &[u8]) -> IResult<&[u8], Import> {
    map(
        (parse_name, parse_name, parse_import_desc),
        |(module, name, desc)| Import { module, name, desc },
    )
    .parse(input)
}

fn parse_import_desc(input: &[u8]) -> IResult<&[u8], ImportDesc> {
    alt((
        map((tag(&[0x00][..]), parse_type_index), |(_, type_index)| {
            ImportDesc::TypeIndex(type_index)
        }),
        map((tag(&[0x01][..]), parse_table_type), |(_, table_type)| {
            ImportDesc::Table(table_type)
        }),
        map((tag(&[0x02][..]), parse_memory_type), |(_, memory_type)| {
            ImportDesc::Memory(memory_type)
        }),
        map((tag(&[0x03][..]), parse_global_type), |(_, global_type)| {
            ImportDesc::Global(global_type)
        }),
    ))
    .parse(input)
}

impl<'a> TryFrom<RawSection<'a>> for FunctionSection {
    type Error = String;

    fn try_from(raw: RawSection<'a>) -> Result<Self, Self::Error> {
        if raw.header.id != SectionID::Function {
            return Err("RawSection is not a FunctionSection".to_string());
        }
        let (_, type_indexes) = length_count(parse_varuint32, parse_type_index)
            .parse(raw.payload)
            .map_err(|e| format!("Failed to parse FunctionSection: {}", e))?;
        Ok(FunctionSection { type_indexes })
    }
}
