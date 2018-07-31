use std::collections::BTreeMap;

use options::{FloatSize, IntSize, Options};
use value::{GenericStruct, GenericValue};

pub type ParsedFields<T> = BTreeMap<String, T>;

pub fn parsed_to_generic_struct<T, F>(
    parsed_config: ParsedFields<T>,
    options: &Options,
    convert_fn: F,
) -> GenericStruct
where
    F: Fn(&str, &str, T, &Options) -> GenericValue,
{
    let struct_name = "Config".to_owned();

    let fields = parsed_config
        .into_iter()
        .map(|(key, value)| {
            let value = convert_fn("_Config", &key, value, options);
            (key, value)
        })
        .collect();

    GenericStruct {
        struct_name,
        fields,
    }
}

pub fn preferred_float(value: f64, preferred: FloatSize) -> GenericValue {
    match preferred {
        FloatSize::F32 => GenericValue::F32(value as f32),
        FloatSize::F64 => GenericValue::F64(value),
    }
}

pub fn preferred_int(value: i64, preferred: IntSize) -> GenericValue {
    match preferred {
        IntSize::I8 => GenericValue::I8(value as i8),
        IntSize::I16 => GenericValue::I16(value as i16),
        IntSize::I32 => GenericValue::I32(value as i32),
        IntSize::I64 => GenericValue::I64(value),
        IntSize::ISize => GenericValue::ISize(value as isize),
    }
}
