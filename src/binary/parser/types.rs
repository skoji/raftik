use super::integer::parse_varuint32;
use crate::ast::types::{
    FunctionType, GlobalType, Limits, Mutability, ReferenceType, TableType, ValueType,
};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::length_count;
use nom::number::complete::u8;
use nom::{IResult, Parser};

pub fn parse_value_type(input: &[u8]) -> IResult<&[u8], ValueType> {
    let (input, value_type_byte) = u8(input)?;
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

pub fn parse_limits(input: &[u8]) -> IResult<&[u8], Limits> {
    alt((
        map((tag(&[0x00][..]), parse_varuint32), |(_, min)| Limits {
            min,
            max: None,
        }),
        map(
            (tag(&[0x01][..]), parse_varuint32, parse_varuint32),
            |(_, min, max)| Limits {
                min,
                max: Some(max),
            },
        ),
    ))
    .parse(input)
}

pub fn parse_table_type(input: &[u8]) -> IResult<&[u8], TableType> {
    map(
        (parse_reference_type, parse_limits),
        |(ref_type, limits)| TableType { ref_type, limits },
    )
    .parse(input)
}

pub fn parse_mutability(input: &[u8]) -> IResult<&[u8], Mutability> {
    alt((
        map(tag(&[0x00][..]), |_| Mutability::Const),
        map(tag(&[0x01][..]), |_| Mutability::Var),
    ))
    .parse(input)
}

pub fn parse_global_type(input: &[u8]) -> IResult<&[u8], GlobalType> {
    map(
        (parse_value_type, parse_mutability),
        |(val_type, mutability)| GlobalType {
            val_type,
            mutability,
        },
    )
    .parse(input)
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

    #[test]
    fn test_parse_limits() {
        let input1 = [0x00, 0x01];
        let expected1 = Limits { min: 1, max: None };
        let result1 = parse_limits(&input1);
        assert_eq!(result1, Ok((&[][..], expected1)));

        let input2 = [0x01, 0x02, 0x03];
        let expected2 = Limits {
            min: 2,
            max: Some(3),
        };
        let result2 = parse_limits(&input2);
        assert_eq!(result2, Ok((&[][..], expected2)));
    }

    #[test]
    fn test_parse_limits_fails() {
        let input1 = [0x02, 0x01]; // Invalid tag for limits
        let result1 = parse_limits(&input1);
        assert!(result1.is_err());
    }

    #[test]
    fn test_parse_table_type() {
        let input = [0x70, 0x00, 0x01];
        let expected = TableType {
            ref_type: ReferenceType::FuncRef,
            limits: Limits { min: 1, max: None },
        };
        let result = parse_table_type(&input);
        assert_eq!(result, Ok((&[][..], expected)));
    }

    #[test]
    fn test_parse_global_type() {
        let input = [0x7f, 0x01]; // I32 with Var mutability
        let expected = GlobalType {
            val_type: ValueType::Number(NumberType::I32),
            mutability: Mutability::Var,
        };
        let result = parse_global_type(&input);
        assert_eq!(result, Ok((&[][..], expected)));
    }
}
