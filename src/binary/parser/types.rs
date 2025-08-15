use super::integer::parse_varuint32;
use crate::ast::types::{FunctionType, ReferenceType, ValueType};
use nom::bytes::complete::tag;
use nom::multi::length_count;
use nom::{IResult, Parser};

pub fn parse_value_type(input: &[u8]) -> IResult<&[u8], ValueType> {
    let (input, value_type_byte) = nom::number::complete::u8(input)?;
    let value_type: ValueType = value_type_byte.try_into().map_err(|_| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::IsNot))
    })?;
    Ok((input, value_type))
}

pub fn parse_function_type(i: &[u8]) -> IResult<&[u8], FunctionType> {
    let (i, _) = tag(&[0x60u8][..])(i)?;
    let (i, params) = length_count(parse_varuint32, parse_value_type).parse(i)?;
    let (i, results) = length_count(parse_varuint32, parse_value_type).parse(i)?;
    Ok((i, FunctionType { params, results }))
}

pub fn parse_reference_type(input: &[u8]) -> IResult<&[u8], ReferenceType> {
    let (input, ref_type_byte) = nom::number::complete::u8(input)?;
    let ref_type = ref_type_byte.try_into().map_err(|_| {
        nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::IsNot))
    })?;
    Ok((input, ref_type))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::types::{NumberType, ReferenceType, ValueType, VectorType};

    #[test]
    fn test_parse_value_type() {
        // expected and result pairs
        let test_cases = vec![
            (0x7f, ValueType::Number(NumberType::I32)),
            (0x7e, ValueType::Number(NumberType::I64)),
            (0x7d, ValueType::Number(NumberType::F32)),
            (0x7c, ValueType::Number(NumberType::F64)),
            (0x7b, ValueType::Vector(VectorType::V128)),
            (0x70, ValueType::Reference(ReferenceType::FuncRef)),
            (0x6f, ValueType::Reference(ReferenceType::ExternRef)),
        ];
        for (input_byte, expected_value_type) in test_cases {
            let input = [input_byte];
            let result = parse_value_type(&input);
            assert_eq!(result, Ok((&[][..], expected_value_type)));
        }
    }

    #[test]
    fn test_parse_function_type() {
        let input = [0x60, 0x02, 0x7f, 0x7e, 0x01, 0x7d];
        let expected = FunctionType {
            params: vec![
                ValueType::Number(NumberType::I32),
                ValueType::Number(NumberType::I64),
            ],
            results: vec![ValueType::Number(NumberType::F32)],
        };
        let result = parse_function_type(&input);
        assert_eq!(result, Ok((&[][..], expected)));
    }

    #[test]
    fn test_parse_reference_type() {
        let test_cases = vec![
            (0x70, ReferenceType::FuncRef),
            (0x6f, ReferenceType::ExternRef),
        ];
        for (input_byte, expected_ref_type) in test_cases {
            let input = [input_byte];
            let result = parse_reference_type(&input);
            assert_eq!(result, Ok((&[][..], expected_ref_type)));
        }
    }
}
