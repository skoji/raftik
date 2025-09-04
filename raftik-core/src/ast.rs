pub mod instructions;
pub mod section;
pub mod types;

pub use section::{
    CodeSection, CustomSection, DataCountSection, DataSection, ElementSection, ExportSection,
    FunctionSection, GlobalSection, ImportSection, MemorySection, Section, StartSection,
    TableSection, TypeSection,
};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct ModuleParsed<'a> {
    pub sections: Vec<Section<'a>>,
}

impl<'a> ModuleParsed<'a> {
    pub fn sec_by_id(&self, id: section::SectionID) -> Option<&Section<'a>> {
        self.sections.iter().find(|s| s.id() == id)
    }
    pub fn sec_by_id_mut(&mut self, id: section::SectionID) -> Option<&mut Section<'a>> {
        self.sections.iter_mut().find(|s| s.id() == id)
    }
}
