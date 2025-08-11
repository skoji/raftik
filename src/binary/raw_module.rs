#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionHeader {
    pub id: u8, 
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

