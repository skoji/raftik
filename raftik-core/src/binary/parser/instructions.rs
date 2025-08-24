use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_until},
    combinator::map,
    sequence::terminated,
};

use crate::ast::instructions::Expression;

const END_OPCODE: u8 = 0x0b;

pub fn parse_expression(input: &[u8]) -> IResult<&[u8], Expression<'_>> {
    map(
        terminated(take_until(&[END_OPCODE][..]), tag(&[END_OPCODE][..])),
        |instructions| Expression { instructions },
    )
    .parse(input)
}
