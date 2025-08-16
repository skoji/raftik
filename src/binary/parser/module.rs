use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, flat_map, map, map_res},
    multi::{length_count, many0},
    number::complete::{le_u32, u8},
};

use super::{
    integer::parse_varuint32,
    name::parse_name,
    section_parser_trait::ParseSection,
    types::{
        parse_function_type, parse_global_type, parse_memory_type, parse_table_type,
        parse_type_index,
    },
};
use crate::ast::{
    Module,
    section::{
        FunctionSection, Import, ImportDesc, ImportSection, Section, SectionID, TypeSection,
        UnknownSection,
    },
};

pub fn parse_module(input: &'_ [u8]) -> IResult<&[u8], Module<'_>> {
    map(
        all_consuming((parse_magic, parse_version, parse_sections)),
        |(_, _, sections)| Module { sections },
    )
    .parse(input)
}

impl ParseSection for TypeSection {
    fn parse_from_payload(payload: &[u8]) -> IResult<&[u8], Self> {
        map(
            length_count(parse_varuint32, parse_function_type),
            |types| TypeSection { types },
        )
        .parse(payload)
    }
}

impl ParseSection for ImportSection {
    fn parse_from_payload(payload: &[u8]) -> IResult<&[u8], Self> {
        map(length_count(parse_varuint32, parse_import), |imports| {
            ImportSection { imports }
        })
        .parse(payload)
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

impl ParseSection for FunctionSection {
    fn parse_from_payload(payload: &[u8]) -> IResult<&[u8], Self> {
        map(
            length_count(parse_varuint32, parse_type_index),
            |type_indexes| FunctionSection { type_indexes },
        )
        .parse(payload)
    }
}

fn parse_magic(input: &[u8]) -> IResult<&[u8], &[u8; 4]> {
    map(tag(&b"\0asm"[..]), |magic: &[u8]| {
        magic.try_into().expect("magic should be exactly 4 bytes")
    })
    .parse(input)
}

fn parse_version(input: &[u8]) -> IResult<&[u8], u32> {
    le_u32(input)
}

fn parse_sections(input: &[u8]) -> IResult<&[u8], Vec<Section<'_>>> {
    many0(parse_section).parse(input)
}

fn parse_section(input: &[u8]) -> IResult<&[u8], Section<'_>> {
    let (input, (id, payload)) =
        (parse_section_id, flat_map(parse_varuint32, take)).parse(input)?;

    let section = match id {
        SectionID::Type => Section::Type(TypeSection::parse_all(payload)?),
        SectionID::Import => Section::Import(ImportSection::parse_all(payload)?),
        SectionID::Function => Section::Function(FunctionSection::parse_all(payload)?),
        _ => Section::Unknown(UnknownSection { id, payload }),
    };
    Ok((input, section))
}

fn parse_section_id(input: &[u8]) -> IResult<&[u8], SectionID> {
    map_res(u8, SectionID::try_from).parse(input)
}
