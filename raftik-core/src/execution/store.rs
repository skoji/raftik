use super::rawinstance::{Func, Module};

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
    pub(super) fn store_module(&mut self, m: Module) -> usize {
        self.modules.push(m);
        self.modules.len() - 1
    }

    pub(super) fn mut_module(&mut self, i: usize) -> Option<&mut Module> {
        self.modules.get_mut(i)
    }

    pub(super) fn store_func(&mut self, f: Func) -> usize {
        self.funcs.push(f);
        self.funcs.len() - 1
    }

    pub(super) fn func(&self, i: usize) -> Option<&Func> {
        self.funcs.get(i)
    }
}
