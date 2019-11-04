use std::collections::BTreeMap;

/// Represents a Rust struct.
#[derive(Debug, Clone)]
pub struct GenericStruct {
    pub struct_name: String,
    pub fields: BTreeMap<String, GenericValue>,
}

/// Represents a typed Rust value.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum GenericValue {
    Unit,
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    ISize(isize),
    Usize(usize),
    F32(f32),
    F64(f64),
    String(String),
    Option(Option<Box<GenericValue>>),
    Array(Vec<GenericValue>),
    Struct(GenericStruct),
}
