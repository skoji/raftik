pub mod section;
pub mod types;

pub use section::{FunctionSection, ImportSection, Section, TableSection, TypeSection};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Module<'a> {
    pub sections: Vec<Section<'a>>,
}
