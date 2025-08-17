use nom::{IResult, Parser, combinator::all_consuming};

pub trait ParseSection<'a>: Sized {
    fn parse_from_payload(payload: &'a [u8]) -> IResult<&'a [u8], Self>;

    fn parse_all(payload: &'a [u8]) -> Result<Self, nom::Err<nom::error::Error<&'a [u8]>>> {
        all_consuming(Self::parse_from_payload)
            .parse(payload)
            .map(|(_, result)| result)
    }
}
