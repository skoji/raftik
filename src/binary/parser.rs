use crate::binary::integer::parse_varuint32;
use crate::binary::raw_module::{RawModule, RawSection, SectionHeader, SectionID};
use nom::{IResult, bytes::complete::tag, number::complete::le_u32};

pub fn parse_module(input: &[u8]) -> IResult<&[u8], RawModule<'_>> {
    let (input, magic) = parse_magic(input)?;
    let (input, version) = parse_version(input)?;
    let (input, sections) = parse_raw_sections(input)?;
    let module = RawModule {
        magic: *magic,
        version,
        sections,
    };

    Ok((input, module))
}

fn parse_magic(input: &[u8]) -> IResult<&[u8], &[u8; 4]> {
    let (input, magic) = tag(&b"\0asm"[..])(input)?;
    Ok((
        input,
        magic
            .try_into()
            .expect("tag ensures slice is exactly 4 bytes long"),
    ))
}

fn parse_version(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, version) = le_u32(input)?;
    Ok((input, version))
}

fn parse_raw_sections(input: &[u8]) -> IResult<&[u8], Vec<RawSection<'_>>> {
    let mut sections = Vec::new();
    let mut remaining_input = input;

    while !remaining_input.is_empty() {
        let (input, section) = parse_raw_section(remaining_input)?;
        sections.push(section);
        remaining_input = input;
    }

    Ok((remaining_input, sections))
}

fn parse_raw_section(input: &[u8]) -> IResult<&[u8], RawSection<'_>> {
    let (input, header) = parse_section_header(input)?;
    let (input, payload) = nom::bytes::complete::take(header.payload_length)(input)?;
    let section = RawSection { header, payload };
    Ok((input, section))
}

fn parse_section_header(input: &[u8]) -> IResult<&[u8], SectionHeader> {
    let (input, id) = parse_section_id(input)?;
    let (input, payload_length) = parse_varuint32(input)?;

    Ok((input, SectionHeader { id, payload_length }))
}

fn parse_section_id(input: &[u8]) -> IResult<&[u8], SectionID> {
    let (input, id_byte) = nom::number::complete::u8(input)?;
    match SectionID::try_from(id_byte) {
        Ok(id) => Ok((input, id)),
        Err(_) => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Alt,
        ))),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

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

    #[test]
    fn test_parse_module() {
        let input = b"\0asm\x01\x00\x00\x00";
        let result = parse_module(input);
        assert_eq!(
            result,
            Ok((
                &b""[..],
                RawModule {
                    magic: *b"\0asm",
                    version: 1,
                    sections: Vec::new(),
                }
            ))
        );
    }
}
