use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_until},
    combinator::map,
    sequence::terminated,
};

use crate::ast::instructions::Expression;

pub fn parse_expression(input: &[u8]) -> IResult<&[u8], Expression<'_>> {
    map(
        terminated(take_until(&[0x0b][..]), tag(&[0x0b][..])),
        |instructions| Expression { instructions },
    )
    .parse(input)
}
