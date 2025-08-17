use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, flat_map, map, map_res},
    multi::{length_count, many0},
    number::complete::{le_u32, u8},
};

use super::{
    instructions::parse_expression,
    integer::parse_varuint32,
    name::parse_name,
    section_parser_trait::ParseSection,
    types::{parse_function_type, parse_global_type, parse_memory_type, parse_table_type},
};
use crate::ast::{
    Module,
    section::{
        Export, ExportDesc, ExportSection, FunctionSection, Global, GlobalSection, Import,
        ImportDesc, ImportSection, MemorySection, Section, SectionID, StartSection, TableSection,
        TypeSection, UnknownSection,
    },
};

pub fn parse_module(input: &'_ [u8]) -> IResult<&[u8], Module<'_>> {
    map(
        all_consuming((parse_magic, parse_version, parse_sections)),
        |(_, _, sections)| Module { sections },
    )
    .parse(input)
}

impl ParseSection<'_> for TypeSection {
    fn parse_from_payload(payload: &[u8]) -> IResult<&[u8], Self> {
        map(
            length_count(parse_varuint32, parse_function_type),
            |types| TypeSection { types },
        )
        .parse(payload)
    }
}

impl ParseSection<'_> for ImportSection {
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
        map((tag(&[0x00][..]), parse_varuint32), |(_, type_index)| {
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

impl ParseSection<'_> for FunctionSection {
    fn parse_from_payload(payload: &[u8]) -> IResult<&[u8], Self> {
        map(
            length_count(parse_varuint32, parse_varuint32),
            |type_indexes| FunctionSection { type_indexes },
        )
        .parse(payload)
    }
}

impl ParseSection<'_> for TableSection {
    fn parse_from_payload(payload: &[u8]) -> IResult<&[u8], Self> {
        map(length_count(parse_varuint32, parse_table_type), |tables| {
            TableSection { tables }
        })
        .parse(payload)
    }
}

impl ParseSection<'_> for MemorySection {
    fn parse_from_payload(payload: &[u8]) -> IResult<&[u8], Self> {
        map(
            length_count(parse_varuint32, parse_memory_type),
            |memories| MemorySection { memories },
        )
        .parse(payload)
    }
}

impl<'a> ParseSection<'a> for GlobalSection<'a> {
    fn parse_from_payload(payload: &'a [u8]) -> IResult<&'a [u8], GlobalSection<'a>> {
        map(length_count(parse_varuint32, parse_global), |globals| {
            GlobalSection { globals }
        })
        .parse(payload)
    }
}

fn parse_global(input: &[u8]) -> IResult<&[u8], Global<'_>> {
    map(
        (parse_global_type, parse_expression),
        |(global_type, expression)| Global {
            global_type,
            expression,
        },
    )
    .parse(input)
}

impl ParseSection<'_> for ExportSection {
    fn parse_from_payload(payload: &[u8]) -> IResult<&[u8], Self> {
        map(length_count(parse_varuint32, parse_export), |exports| {
            ExportSection { exports }
        })
        .parse(payload)
    }
}

fn parse_export(input: &[u8]) -> IResult<&[u8], Export> {
    map((parse_name, parse_export_desc), |(name, desc)| Export {
        name,
        desc,
    })
    .parse(input)
}

fn parse_export_desc(input: &[u8]) -> IResult<&[u8], ExportDesc> {
    map_res((u8, parse_varuint32), |(id, index)| match id {
        0 => Ok(ExportDesc::FunctionIndex(index)),
        1 => Ok(ExportDesc::TableIndex(index)),
        2 => Ok(ExportDesc::MemoryIndex(index)),
        3 => Ok(ExportDesc::GlobalIndex(index)),
        _ => Err(nom::error::Error::<&[u8]> {
            input,
            code: nom::error::ErrorKind::Alt,
        }),
    })
    .parse(input)
}

impl ParseSection<'_> for StartSection {
    fn parse_from_payload(payload: &[u8]) -> IResult<&[u8], Self> {
        map(parse_varuint32, |start_function_index| StartSection {
            start_function_index,
        })
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
        SectionID::Table => Section::Table(TableSection::parse_all(payload)?),
        SectionID::Memory => Section::Memory(MemorySection::parse_all(payload)?),
        SectionID::Global => Section::Global(GlobalSection::parse_all(payload)?),
        SectionID::Export => Section::Export(ExportSection::parse_all(payload)?),
        SectionID::Start => Section::Start(StartSection::parse_all(payload)?),
        _ => Section::Unknown(UnknownSection { id, payload }),
    };
    Ok((input, section))
}

fn parse_section_id(input: &[u8]) -> IResult<&[u8], SectionID> {
    map_res(u8, SectionID::try_from).parse(input)
}
