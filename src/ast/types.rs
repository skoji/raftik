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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionType {
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>,
}
