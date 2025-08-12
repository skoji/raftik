#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SectionID {
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

impl TryFrom<u8> for SectionID {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Custom),
            1 => Ok(Self::Type),
            2 => Ok(Self::Import),
            3 => Ok(Self::Function),
            4 => Ok(Self::Table),
            5 => Ok(Self::Memory),
            6 => Ok(Self::Global),
            7 => Ok(Self::Export),
            8 => Ok(Self::Start),
            9 => Ok(Self::Element),
            10 => Ok(Self::Code),
            11 => Ok(Self::Data),
            12 => Ok(Self::DataCount),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionHeader {
    pub id: SectionID,
    pub payload_length: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawSection<'a> {
    pub header: SectionHeader,
    pub payload: &'a [u8],
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawModule<'a> {
    pub magic: [u8; 4],
    pub version: u32,
    pub sections: Vec<RawSection<'a>>,
}
