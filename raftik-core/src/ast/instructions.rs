#[derive(Debug, PartialEq, Eq)]
pub struct RawExpression<'a> {
    pub instructions: &'a [u8],
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Opcode {
    LocalGet(u32),
    I32Add,
}
