use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error(
        "index {referred_index} out of bounds in {referred} section (referenced from {referring} section at index {referring_index})"
    )]
    IndexOutOfBoundsIn {
        referring: &'static str,
        referring_index: usize,
        referred: &'static str,
        referred_index: u32,
    },
    #[error(
        "code section length mismatch: functions declared: {funcs_declared}, code bodies: {code_bodies}"
    )]
    CodeSectionLengthMismatch {
        funcs_declared: usize,
        code_bodies: usize,
    },
    #[error("control stack underflow")]
    ControlStackUnderflow,

    #[error("value stack underflow")]
    ValueStackUnderflow,

    #[error("pop value expected {expected:?}, actual  {actual:?}")]
    PopValueTypeMismatch {
        expected: crate::ast::types::ValueType,
        actual: crate::ast::types::ValueType,
    },
}
