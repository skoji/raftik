use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::{flat_map, map_res},
};

use super::integer::parse_varuint32;

pub fn parse_name(i: &[u8]) -> IResult<&[u8], String> {
    map_res(flat_map(parse_varuint32, take), |b: &[u8]| {
        std::str::from_utf8(b).map(|s| s.to_string())
    })
    .parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_name() {
        let input = [
            0x05, b'H', b'e', b'l', b'l', b'o', b'W', b'o', b'r', b'l', b'd',
        ];
        let result = parse_name(&input);
        assert_eq!(
            result,
            Ok((&[b'W', b'o', b'r', b'l', b'd'][..], "Hello".to_string()))
        );
    }

    #[test]
    fn test_parse_name_incomplete() {
        let input = [0x05, b'H', b'e', b'l', b'l'];
        let result = parse_name(&input);
        assert!(result.is_err(), "Expected error for incomplete name");
    }
}
