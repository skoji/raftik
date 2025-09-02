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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OpcodeCategory {
    Variable,
    Reference,
    NumericConst,
    Numeric,
}

impl Opcode {
    pub fn is_constant(&self) -> bool {
        matches!(
            self,
            Opcode::GlobalGet(..)
                | Opcode::I32Const(..)
                | Opcode::I64Const(..)
                | Opcode::F32Const(..)
                | Opcode::F64Const(..)
                | Opcode::RefNull(..)
                | Opcode::RefFunc(..)
        )
    }

    pub fn category(&self) -> OpcodeCategory {
        match self {
            Opcode::LocalGet(_)
            | Opcode::LocalSet(_)
            | Opcode::LocalTee(_)
            | Opcode::GlobalGet(_)
            | Opcode::GlobalSet(_) => OpcodeCategory::Variable,
            Opcode::I32Const(_)
            | Opcode::I64Const(_)
            | Opcode::F32Const(_)
            | Opcode::F64Const(_) => OpcodeCategory::NumericConst,
            Opcode::RefFunc(_) | Opcode::RefNull(_) | Opcode::RefIsNull => {
                OpcodeCategory::Reference
            }
            Opcode::I32Add => OpcodeCategory::Numeric,
        }
    }
}
