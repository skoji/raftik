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

    #[error("instruction validation error")]
    InstructionValidationError {
        desc: String,
        error: VInstError,
        progress: Vec<crate::ast::instructions::Opcode>,
        value_stack: Vec<crate::validation::instruction::StackValue>,
        control_stack: Vec<crate::validation::instruction::ControlFrame>,
    },

    #[error("table is invalid: index {index}, Limits: {limits:?}, system maximum: {maximum}")]
    TableSizeError {
        index: usize,
        limits: crate::ast::types::Limits,
        maximum: u32,
    },

    #[error("memory is invalid: index {index}, Limits: {limits:?}, system maximum: {maximum}")]
    MemorySizeError {
        index: usize,
        limits: crate::ast::types::Limits,
        maximum: u32,
    },
}

#[derive(Error, Debug)]
pub enum VInstError {
    #[error("control stack underflow")]
    ControlStackUnderflow,

    #[error("value stack underflow")]
    ValueStackUnderflow,

    #[error("pop value expected {expected:?}, actual  {actual:?}")]
    PopValueTypeMismatch {
        expected: crate::ast::types::ValueType,
        actual: crate::ast::types::ValueType,
    },

    #[error("opcode parse failed: {0}")]
    OpcodeParseFailed(String),

    #[error("no local found at index {0}")]
    NoLocalAtIndex(u32),

    #[error("no global found at index {0}")]
    NoGlobalAtIndex(u32),

    #[error("no function found at index {0}")]
    NoFunctionAtIndex(u32),

    #[error("opcode should be constant: {0:?}")]
    OpcodeShouldBeConstant(crate::ast::instructions::Opcode),

    #[error("opcode should be constants: reffering not constant; global get {0}")]
    GlobalGetShouldBeConstant(u32),

    #[error("stack value should be reference type, actual: {0:?}")]
    StackValueShouldBeRefType(crate::validation::instruction::StackValue),
}
