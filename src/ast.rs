pub mod section;
pub mod types;

pub use section::TypeSection;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Module {
    pub type_section: Option<TypeSection>,
}
