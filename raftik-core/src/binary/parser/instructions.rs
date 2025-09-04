use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::map,
    number::{le_f32, le_f64},
    sequence::terminated,
};

use super::{
    integer::{parse_varint32, parse_varint64, parse_varuint32},
    types::parse_reference_type,
};
use crate::ast::instructions::{Opcode, RawExpression};

const END_OPCODE: u8 = 0x0b;

pub fn parse_expression(input: &[u8]) -> IResult<&[u8], RawExpression<'_>> {
    map(
        terminated(take_until(&[END_OPCODE][..]), tag(&[END_OPCODE][..])),
        |instructions| RawExpression { instructions },
    )
    .parse(input)
}

fn parse_numeric_const(input: &[u8]) -> IResult<&[u8], Opcode> {
    alt((
        map((tag(&[0x41][..]), parse_varint32), |(_, v)| {
            Opcode::I32Const(v)
        }),
        map((tag(&[0x42][..]), parse_varint64), |(_, v)| {
            Opcode::I64Const(v)
        }),
        map((tag(&[0x43][..]), le_f32()), |(_, v)| Opcode::F32Const(v)),
        map((tag(&[0x44][..]), le_f64()), |(_, v)| Opcode::F64Const(v)),
    ))
    .parse(input)
}

macro_rules! variable_instruction {
    ($($b:literal => $v:ident),+ $(,)?) => {
        alt((
            $(
                map((tag(&[$b][..]), parse_varuint32), |(_, v)|  Opcode::$v(v)),
            )+
        ))
    };
}

fn parse_variable_instruction(input: &[u8]) -> IResult<&[u8], Opcode> {
    variable_instruction! {
        0x20 => LocalGet,
        0x21 => LocalSet,
        0x22 => LocalTee,
        0x23 => GlobalGet,
        0x24 => GlobalSet,
    }
    .parse(input)
}

fn parse_reference_instruction(input: &[u8]) -> IResult<&[u8], Opcode> {
    alt((
        map((tag(&[0xD0][..]), parse_reference_type), |(_, r)| {
            Opcode::RefNull(r)
        }),
        map(tag(&[0xD1][..]), |_| Opcode::RefIsNull),
        map((tag(&[0xD2][..]), parse_varuint32), |(_, i)| {
            Opcode::RefFunc(i)
        }),
    ))
    .parse(input)
}

pub fn parse_instruction(input: &[u8]) -> IResult<&[u8], Opcode> {
    alt((
        parse_reference_instruction,
        parse_numeric_const,
        parse_variable_instruction,
        map(tag(&[0x6a][..]), |_| Opcode::I32Add),
    ))
    .parse(input)
}
