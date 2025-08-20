pub mod instructions;
pub mod section;
pub mod types;

pub use section::{
    CodeSection, DataCountSection, DataSection, ElementSection, ExportSection, FunctionSection,
    GlobalSection, ImportSection, MemorySection, Section, StartSection, TableSection, TypeSection,
};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Module<'a> {
    pub sections: Vec<Section<'a>>,
}
