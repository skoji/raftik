use crate::ast::{
    instructions::Opcode,
    types::{FunctionType, ValueType},
};

#[derive(Debug, Clone)]
pub struct ModuleInstance {
    pub funcs: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct WasmFuncInstance<'a> {
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

#[derive(Debug, Clone)]
pub enum FuncInstance<'a> {
    Wasm(WasmFuncInstance<'a>),
    External(ExternalFuncInstance),
}

#[derive(Debug, Clone)]
pub struct ExportInstance {
    pub name: String,
    pub value: ExportValue,
}

#[derive(Debug, Clone)]
pub enum ExportValue {
    Func(usize),
}
