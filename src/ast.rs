pub mod section;
pub mod types;

pub use section::{FunctionSection, ImportSection, Section, TypeSection};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Module {
    pub sections: Vec<Section>,
}
