use std::collections::BTreeMap;

use value::{GenericStruct, GenericValue};

pub type ParsedFields<T> = BTreeMap<String, T>;

pub fn parsed_to_generic_struct<T, F>(
    parsed_config: ParsedFields<T>,
    convert_fn: F,
) -> GenericStruct
where
    F: Fn(&str, &str, T) -> GenericValue,
{
    let struct_name = "Config".to_owned();

    let fields = parsed_config
        .into_iter()
        .map(|(key, value)| {
            let value = convert_fn("_Config", &key, value);
            (key, value)
        })
        .collect();

    GenericStruct {
        struct_name,
        fields,
    }
}
