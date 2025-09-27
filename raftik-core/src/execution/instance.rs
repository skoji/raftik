use crate::ast::{
    instructions::Opcode,
    types::{FunctionType, ValueType},
};

#[derive(Debug, Clone)]
pub struct ModuleInstance {
    pub func_addresses: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct WasmFuncInstance {
    pub t: FunctionType,
    pub module_address: usize,
    pub locals: Vec<ValueType>,
    pub body: Vec<Opcode>,
}

#[derive(Debug, Clone)]
pub struct ExternalFuncInstance {
    pub t: FunctionType,
    // TODO; expression of body implementation
}

#[derive(Debug, Clone)]
pub enum FuncInstance {
    Wasm(WasmFuncInstance),
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
