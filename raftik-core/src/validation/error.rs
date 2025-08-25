use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Function Section: No Type Section found")]
    NoTypeSectionInFunctionSection,
    #[error("Function Section: Type index {0} out of bounds at {1} for Type Section length {2}")]
    TypeIndexOutOfBoundsInFunctionSection(u32, usize, usize),
}
