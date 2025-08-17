#[derive(Debug, PartialEq, Eq)]
pub struct Expression<'a> {
    pub instructions: &'a [u8],
}
