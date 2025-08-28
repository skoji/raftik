use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::map,
    sequence::terminated,
};

use super::integer::parse_varuint32;
use crate::ast::instructions::{Opcode, RawExpression};

const END_OPCODE: u8 = 0x0b;

pub fn parse_expression(input: &[u8]) -> IResult<&[u8], RawExpression<'_>> {
    map(
        terminated(take_until(&[END_OPCODE][..]), tag(&[END_OPCODE][..])),
        |instructions| RawExpression { instructions },
    )
    .parse(input)
}

pub fn parse_instruction(input: &[u8]) -> IResult<&[u8], Opcode> {
    alt((
        map((tag(&[0x20][..]), parse_varuint32), |(_, i)| {
            Opcode::LocalGet(i)
        }),
        map(tag(&[0x6a][..]), |_| Opcode::I32Add),
    ))
    .parse(input)
}
