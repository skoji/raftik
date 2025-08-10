use super::leb128::{decode_uleb128_u64, decode_sleb128_i64, Leb128Err};

use nom::{IResult, Err as NomErr};
use nom::error::{ErrorKind, ParseError};

pub fn parse_varuint32<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], u32, E> {
    match decode_uleb128_u64(i, 5, 32) { // ceil(32/7)=5
        Ok((v, used)) => {
            let v32 = u32::try_from(v)
                .map_err(|_| NomErr::Error(E::from_error_kind(i, ErrorKind::TooLarge)))?;
            Ok((&i[used..], v32))
        }
        Err(Leb128Err::Unterminated) => Err(NomErr::Incomplete(nom::Needed::Unknown)),
        Err(_) => Err(NomErr::Error(E::from_error_kind(i, ErrorKind::Fail))),
    }
}

pub fn parse_varuint64<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], u64, E> {
    match decode_uleb128_u64(i, 10, 64) { // ceil(64/7)=10
        Ok((v, used)) => Ok((&i[used..], v)),
        Err(Leb128Err::Unterminated) => Err(NomErr::Incomplete(nom::Needed::Unknown)),
        Err(_) => Err(NomErr::Error(E::from_error_kind(i, ErrorKind::Fail))),
    }
}

pub fn parse_varint32<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], i32, E> {
    match decode_sleb128_i64(i, 5, 32) { // ceil(32/7)=5
        Ok((v, used)) => {
            let v32 = i32::try_from(v)
                .map_err(|_| NomErr::Error(E::from_error_kind(i, ErrorKind::TooLarge)))?;
            Ok((&i[used..], v32))
        }
        Err(Leb128Err::Unterminated) => Err(NomErr::Incomplete(nom::Needed::Unknown)),
        Err(_) => Err(NomErr::Error(E::from_error_kind(i, ErrorKind::Fail))),
    }
}

pub fn parse_varint64<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], i64, E> {
    match decode_sleb128_i64(i, 10, 64) { // ceil(64/7)=10
        Ok((v, used)) => Ok((&i[used..], v)),
        Err(Leb128Err::Unterminated) => Err(NomErr::Incomplete(nom::Needed::Unknown)),
        Err(_) => Err(NomErr::Error(E::from_error_kind(i, ErrorKind::Fail))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_varuint32() {
        let input = [0xE5, 0x8E, 0x26];
        let result = parse_varuint32::<nom::error::Error<&[u8]>>(&input);
        assert_eq!(result, Ok((&[][..], 624485)));
    }

    #[test]
    fn test_parse_varuint64() {
        let input = [0xE5, 0x8E, 0x26];
        let result = parse_varuint64::<nom::error::Error<&[u8]>>(&input);
        assert_eq!(result, Ok((&[][..], 624485)));
    }

    #[test]
    fn test_parse_varint32() {
        let input = [0x7f];
        let result = parse_varint32::<nom::error::Error<&[u8]>>(&input);
        assert_eq!(result, Ok((&[][..], -1)));
    }

    #[test]
    fn test_parse_varint64() {
        let input = [0x7f];
        let result = parse_varint64::<nom::error::Error<&[u8]>>(&input);
        assert_eq!(result, Ok((&[][..], -1)));
    }
}

