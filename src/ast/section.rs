use crate::ast::types::FunctionType;

#[derive(Debug, PartialEq, Eq)]
pub enum Section {
    Type(TypeSection),
    // Other section coming
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeSection {
    pub types: Vec<FunctionType>,
}
