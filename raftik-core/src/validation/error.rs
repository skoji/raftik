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
    #[error("No Table Section found for Export Section at {0}")]
    NoTableSectionInExportSection(usize),
    #[error("Export Section: Table index {0} out of bounds at {1} for Table Section")]
    TableIndexOutOfBoundsInExportSection(u32, usize),
    #[error("No Global Section found for Export Section at {0}")]
    NoGlobalSectionInExportSection(usize),
    #[error("Export Section: Global index {0} out of bounds at {1} for Global Section")]
    GlobalIndexOutOfBoundsInExportSection(u32, usize),
    #[error("No Memory Section found for Export Section at {0}")]
    NoMemorySectionInExportSection(usize),
    #[error("Export Section: Memory index {0} out of bounds at {1} for Memory Section")]
    MemoryIndexOutOfBoundsInExportSection(u32, usize),
}
