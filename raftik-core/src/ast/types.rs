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
            0x7F => Ok(NumberType::I32.into()),
            0x7E => Ok(NumberType::I64.into()),
            0x7D => Ok(NumberType::F32.into()),
            0x7C => Ok(NumberType::F64.into()),
            0x7B => Ok(VectorType::V128.into()),
            0x70 => Ok(ReferenceType::FuncRef.into()),
            0x6F => Ok(ReferenceType::ExternRef.into()),
            _ => Err("Invalid ValueType"),
        }
    }
}

impl From<NumberType> for ValueType {
    fn from(value: NumberType) -> Self {
        ValueType::Number(value)
    }
}

impl From<VectorType> for ValueType {
    fn from(value: VectorType) -> Self {
        ValueType::Vector(value)
    }
}

impl From<ReferenceType> for ValueType {
    fn from(value: ReferenceType) -> Self {
        ValueType::Reference(value)
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
