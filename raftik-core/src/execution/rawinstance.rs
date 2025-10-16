use crate::ast::{
    instructions::Opcode,
    types::{FunctionType, ValueType},
};

#[derive(Debug, Clone, Default)]
pub struct Module {
    pub func_addresses: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct WasmFunc {
    pub t: FunctionType,
    pub module_address: usize,
    pub locals: Vec<ValueType>,
    pub body: Vec<Opcode>,
}

#[derive(Debug, Clone)]
pub struct ExternalFunc {
    pub t: FunctionType,
    // TODO; expression of body implementation
}

#[derive(Debug, Clone)]
pub enum Func {
    Wasm(WasmFunc),
    External(ExternalFunc),
}

#[derive(Debug, Clone)]
pub struct Export {
    pub name: String,
    pub value: ExportValue,
}

#[derive(Debug, Clone)]
pub enum ExportValue {
    Func(usize),
}
