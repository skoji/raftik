use super::{rawinstance::Module as RawModule, store::Store};

struct Module {
    index: usize,
}

impl Module {
    fn new(_wat: &[u8], store: &mut Store) -> Self {
        let raw_module = RawModule::default();
        let index = store.register_module(raw_module);
        Module { index }
    }
}
