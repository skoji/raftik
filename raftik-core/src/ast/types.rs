#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Number(NumberType),
    Vector(VectorType),
    Reference(ReferenceType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberType {
    I32 = 0x7f,
    I64 = 0x7e,
    F32 = 0x7d,
    F64 = 0x7c,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorType {
    V128 = 0x7b,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceType {
    FuncRef = 0x70,
    ExternRef = 0x6f,
}

impl TryFrom<u8> for ValueType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x7F => Ok(ValueType::Number(NumberType::I32)),
            0x7E => Ok(ValueType::Number(NumberType::I64)),
            0x7D => Ok(ValueType::Number(NumberType::F32)),
            0x7C => Ok(ValueType::Number(NumberType::F64)),
            0x7B => Ok(ValueType::Vector(VectorType::V128)),
            0x70 => Ok(ValueType::Reference(ReferenceType::FuncRef)),
            0x6F => Ok(ValueType::Reference(ReferenceType::ExternRef)),
            _ => Err("Invalid ValueType"),
        }
    }
}

impl TryFrom<u8> for ReferenceType {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x70 => Ok(ReferenceType::FuncRef),
            0x6F => Ok(ReferenceType::ExternRef),
            _ => Err("Invalid ReferenceType"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionType {
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryType {
    pub limits: Limits,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableType {
    pub ref_type: ReferenceType,
    pub limits: Limits,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mutability {
    Const,
    Var,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalType {
    pub val_type: ValueType,
    pub mutability: Mutability,
}
