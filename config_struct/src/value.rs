use std::collections::BTreeMap;

/// Represents a typed Rust value.
#[derive(Debug, Clone)]
pub enum RawValue {
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
    Isize(isize),
    Usize(usize),
    F32(f32),
    F64(f64),
    String(String),
    Option(Option<Box<RawValue>>),
    Array(Vec<RawValue>),
    Struct(RawStructValue),
}

/// Represents a Rust struct.
#[derive(Debug, Clone)]
pub struct RawStructValue {
    pub struct_name: String,
    pub fields: BTreeMap<String, RawValue>,
}

/// Represents a parsed config struct.
#[derive(Debug, Clone)]
pub struct ParsedConfig {
    pub filename: Option<String>,
    pub struct_value: RawStructValue,
    pub markup: MarkupLanguage,
}

/// Represents one of the supported markup languages.
#[derive(Debug, Clone, Copy)]
pub enum MarkupLanguage {
    Json,
    Ron,
    Toml,
    Yaml,
}
