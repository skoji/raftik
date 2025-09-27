use super::instance::{FuncInstance, ModuleInstance};

#[derive(Debug, Clone, Default)]
pub struct Store {
    modules: Vec<ModuleInstance>,
    funcs: Vec<FuncInstance>,
}
