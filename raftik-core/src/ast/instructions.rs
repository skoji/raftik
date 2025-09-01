use super::types::ReferenceType;

#[derive(Debug, PartialEq, Eq)]
pub struct RawExpression<'a> {
    pub instructions: &'a [u8],
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    RefNull(ReferenceType),
    RefIsNull,
    RefFunc(u32),
    I32Add,
}

impl Opcode {
    pub fn is_constant(&self) -> bool {
        match self {
            Opcode::GlobalGet(..) => true,
            Opcode::I32Const(..) => true,
            Opcode::I64Const(..) => true,
            Opcode::F32Const(..) => true,
            Opcode::F64Const(..) => true,
            Opcode::RefNull(..) => true,
            Opcode::RefFunc(..) => true,
            _ => false,
        }
    }
}
