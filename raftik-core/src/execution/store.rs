use super::{
    error::Error,
    rawinstance::{Func, Module},
};
use crate::ast::ModuleParsed;

#[derive(Debug, Clone, Default)]
pub struct Store {
    modules: Vec<Module>,
    funcs: Vec<Func>,
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    // internal APIs
    pub(super) fn register_module_from_wat(&mut self, wat: &[u8]) -> Result<usize, Error> {
        let m = Module::from_slice(wat, self)?;
        self.modules.push(m);
        Ok(self.modules.len() - 1)
    }

    pub(super) fn mut_module(&mut self, i: usize) -> Option<&mut Module> {
        self.modules.get_mut(i)
    }

    pub(super) fn register_func(&mut self, f: Func) -> usize {
        self.funcs.push(f);
        self.funcs.len() - 1
    }

    pub(super) fn func(&self, i: usize) -> Option<&Func> {
        self.funcs.get(i)
    }
}
