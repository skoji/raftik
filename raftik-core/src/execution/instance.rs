use crate::ast::{
    instructions::Opcode,
    types::{FunctionType, ValueType},
};

#[derive(Debug, Clone)]
pub struct ModuleInstance {
    pub funcs: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct FuncInstance<'a> {
    pub t: FunctionType,
    pub module: &'a ModuleInstance,
    pub locals: Vec<ValueType>,
    pub body: Vec<Opcode>,
}

#[derive(Debug, Clone)]
pub struct ExternalFuncInstance {
    pub t: FunctionType,
    // TODO; expression of body implementation
}
