use nom::{
    IResult, Parser,
    bytes::complete::tag,
    combinator::{flat_map, map, map_res},
    multi::many0,
    number::complete::le_u32,
};

use super::integer::parse_varuint32;
use crate::binary::raw_module::{RawModule, RawSection, SectionHeader, SectionID};

pub fn parse_raw_module(input: &[u8]) -> IResult<&[u8], RawModule<'_>> {
    map(
        (parse_magic, parse_version, parse_raw_sections),
        |(magic, version, sections)| RawModule {
            magic: *magic,
            version,
            sections,
        },
    )
    .parse(input)
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

fn parse_raw_sections(input: &[u8]) -> IResult<&[u8], Vec<RawSection<'_>>> {
    many0(parse_raw_section).parse(input)
}

fn parse_raw_section(input: &[u8]) -> IResult<&[u8], RawSection<'_>> {
    flat_map(parse_section_header, |header| {
        map(
            nom::bytes::complete::take(header.payload_length),
            move |payload| RawSection { header, payload },
        )
    })
    .parse(input)
}

fn parse_section_header(input: &[u8]) -> IResult<&[u8], SectionHeader> {
    map(
        (parse_section_id, parse_varuint32),
        |(id, payload_length)| SectionHeader { id, payload_length },
    )
    .parse(input)
}

fn parse_section_id(input: &[u8]) -> IResult<&[u8], SectionID> {
    map_res(nom::number::complete::u8, SectionID::try_from).parse(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::binary::raw_module::*;

    #[test]
    fn test_minimal_wasm() {
        let wasm = wat::parse_str("(module)").unwrap();
        let (_, module) = parse_raw_module(wasm.as_ref()).unwrap();
        assert_eq!(module.magic, [0x00, 0x61, 0x73, 0x6d]);
        assert_eq!(module.version, 1);
        assert_eq!(module.sections.len(), 0);
    }

    #[test]
    fn test_module_with_only_type_section() {
        let wasm = wat::parse_str(
            "(module
                (type (func))
            )",
        )
        .unwrap();
        let (_, module) = parse_raw_module(wasm.as_ref()).unwrap();
        assert_eq!(module.magic, [0x00, 0x61, 0x73, 0x6d]);
        assert_eq!(module.version, 1);
        assert_eq!(module.sections.len(), 1);
        assert_eq!(module.sections[0].header.id, SectionID::Type);
        assert_eq!(module.sections[0].header.payload_length, 4);
    }

    #[test]
    fn test_module_try_from_short_data() {
        let data: &[u8] = &[0x4d, 0x4f];
        let result = parse_raw_module(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_magic() {
        let input = b"\0asm";
        let result = parse_magic(input);
        assert_eq!(result, Ok((&b""[..], b"\0asm")));
    }

    #[test]
    fn test_parse_magic_fails() {
        let input = b"\0as";
        let result = parse_magic(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_version() {
        let input = &[0x01, 0x00, 0x00, 0x00];
        let result = parse_version(input);
        assert_eq!(result, Ok((&b""[..], 1)));
    }
    #[test]
    fn test_parse_version_fails() {
        let input = &[0x01, 0x00, 0x00];
        let result = parse_version(input);
        assert!(result.is_err());
    }
}
