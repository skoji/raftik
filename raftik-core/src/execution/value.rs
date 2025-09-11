#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vector {
    V128(i128),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reference {
    RefFunc(Option<usize>),
    RefExtern(Option<usize>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Num(Number),
    Vec(Vector),
    Ref(Reference),
}
