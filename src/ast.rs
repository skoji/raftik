use crate::ast::section::Section;

pub mod section;
pub mod types;

#[derive(Debug, PartialEq, Eq)]
pub struct Module<'a> {
    pub magic: [u8; 4],
    pub version: u32,
    pub sections: Vec<Section<'a>>,
}
