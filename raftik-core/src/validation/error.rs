use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Function Section: No Type Section found")]
    NoTypeSectionInFunctionSection,
    #[error("Function Section: Type index {0} out of bounds at {1} for Type Section")]
    TypeIndexOutOfBoundsInFunctionSection(u32, usize),
    #[error("No Function Section found for Export Section at {0}")]
    NoFunctionSectionInExportSection(usize),
    #[error("Export Section: Function index {0} out of bounds at {1} for Function Section")]
    FunctionIndexOutOfBoundsInExportSection(u32, usize),
}
