use crate::ast::types::FunctionType;

#[derive(Debug, PartialEq, Eq)]
pub enum SectionId {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
    DataCount = 12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionHeader {
    pub id: u8, // use u8 directly here to handle unknown sections
    pub payload_length: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawSection<'a> {
    pub header: SectionHeader,
    pub payload: &'a [u8],
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeSection {
    pub types: Vec<FunctionType>,
}
