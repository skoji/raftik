use crate::ast::types::{FunctionType, GlobalType, MemoryType, TableType, TypeIndex};

#[derive(Debug, PartialEq, Eq)]
pub enum Section {
    Type(TypeSection),
    Import(ImportSection),
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
