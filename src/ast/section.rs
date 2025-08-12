use crate::ast::types::FunctionType;

#[derive(Debug, PartialEq, Eq)]
pub struct TypeSection {
    pub types: Vec<FunctionType>,
}
