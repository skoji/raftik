use crate::ast::Module;
use nom::{bytes::complete::tag, number::complete::le_u32, IResult};

pub fn parse_module(input: &[u8]) -> IResult<&[u8], Module> {
    let (input, magic) = parse_magic(input)?;
    let (input, version) = parse_version(input)?;
    let module = Module {
        magic: *magic,
        version,
    };

    Ok((input, module))
}

fn parse_magic(input: &[u8]) -> IResult<&[u8], &[u8; 4]> {
    let (input, magic) = tag(&b"\0asm"[..])(input)?;
    Ok((input, magic.try_into().unwrap()))
}

fn parse_version(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, version) = le_u32(input)?;
    Ok((input, version))
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
                Module {
                    magic: *b"\0asm",
                    version: 1
                }
            ))
        );
    }
}
