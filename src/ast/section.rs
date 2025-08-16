use crate::ast::types::{FunctionType, GlobalType, MemoryType, TableType, TypeIndex};

#[derive(Debug, PartialEq, Eq)]
pub enum Section<'a> {
    Type(TypeSection),
    Import(ImportSection),
    Function(FunctionSection),
    Unknown(UnknownSection<'a>),
    // Other section coming
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeSection {
    pub types: Vec<FunctionType>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImportSection {
    pub imports: Vec<Import>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub desc: ImportDesc,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImportDesc {
    TypeIndex(TypeIndex),
    Table(TableType),
    Memory(MemoryType),
    Global(GlobalType),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionSection {
    pub type_indexes: Vec<TypeIndex>,
}

// will be removed all section parser is implemented.
#[derive(Debug, PartialEq, Eq)]
pub struct UnknownSection<'a> {
    pub id: u32,
    pub payload: &'a [u8],
}
