#[derive(Debug, PartialEq, Eq)]
pub struct Module {
    pub magic: [u8; 4],
    pub version: u32,
}
