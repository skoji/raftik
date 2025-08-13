use super::RawSection;
use crate::ast::section::TypeSection;
use crate::binary::raw_module::SectionID;

impl<'a> TryFrom<RawSection<'a>> for TypeSection {
    type Error = String;

    fn try_from(raw: RawSection<'a>) -> Result<Self, Self::Error> {
        if raw.header.id != SectionID::Type {
            return Err("RawSection is not a TypeSection".to_string());
        }

        // Here you would parse the payload into the TypeSection structure
        // For now, we will just return an empty TypeSection
        Ok(TypeSection {
            types: Vec::new(), // Placeholder for actual type parsing logic
        })
    }
}
