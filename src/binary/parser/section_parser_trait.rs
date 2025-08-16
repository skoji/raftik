use nom::{IResult, Parser, combinator::all_consuming};

pub trait ParseSection: Sized {
    fn parse_from_payload(payload: &[u8]) -> IResult<&[u8], Self>;

    fn parse_all(payload: &[u8]) -> Result<Self, nom::Err<nom::error::Error<&[u8]>>> {
        all_consuming(Self::parse_from_payload)
            .parse(payload)
            .map(|(_, result)| result)
    }
}
