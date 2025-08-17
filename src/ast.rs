pub mod instructions;
pub mod section;
pub mod types;

pub use section::{
    FunctionSection, GlobalSection, ImportSection, MemorySection, Section, TableSection,
    TypeSection,
};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Module<'a> {
    pub sections: Vec<Section<'a>>,
}
