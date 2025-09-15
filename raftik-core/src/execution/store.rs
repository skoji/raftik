use super::instance::FuncInstance;

#[derive(Debug, Clone, Default)]
pub struct Store<'a> {
    pub funcs: Vec<FuncInstance<'a>>,
}
