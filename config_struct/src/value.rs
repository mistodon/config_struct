use std::collections::BTreeMap;


pub enum RawValue
{
    Unit,
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Isize(isize),
    Usize(usize),
    F32(f32),
    F64(f64),
    String(String),
    Array(Vec<RawValue>),
    Struct(RawStructValue)
}

pub struct RawStructValue
{
    pub struct_name: String,
    pub fields: BTreeMap<String, RawValue>
}

