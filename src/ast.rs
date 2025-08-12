pub mod section;
pub mod types;

use section::TypeSection;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Module {
    pub type_section: Option<TypeSection>,
}
