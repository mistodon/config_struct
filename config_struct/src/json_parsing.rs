//! Parsing utilities for JSON config files. (Requires the `json-parsing` feature.)

use std::path::Path;

use failure::Error;
use serde_json::{self, Value};

use {ParsedConfig, MarkupLanguage, RawStructValue, RawValue};

/// Parse a ParsedConfig from some JSON.
///
/// This can then be used to generate a config struct using `create_config_module` or
/// `write_config_module`.
pub fn parse_config<S: AsRef<str>>(config_source: S) -> Result<ParsedConfig, Error> {
    use parsing::{self, ParsedFields};

    let json_object: ParsedFields<Value> = serde_json::from_str(config_source.as_ref())?;

    let raw_config = parsing::parsed_to_raw_config(json_object, json_to_raw_value);

    Ok(ParsedConfig { filename: None, struct_value: raw_config, markup: MarkupLanguage::Json })
}

/// Parse a ParsedConfig from a JSON file.
///
/// This can then be used to generate a config struct using `create_config_module` or
/// `write_config_module`.
pub fn parse_config_from_file<P: AsRef<Path>>(config_path: P) -> Result<ParsedConfig, Error> {
    use parsing;

    let config_source = parsing::slurp_file(config_path.as_ref())?;

    parse_config(&config_source)
}

fn json_to_raw_value(super_struct: &str, super_key: &str, value: Value) -> RawValue {
    match value {
        Value::Null => RawValue::Option(None),
        Value::Bool(value) => RawValue::Bool(value),
        Value::Number(value) => match (value.as_i64(), value.as_u64(), value.as_f64()) {
            (Some(x), _, _) => RawValue::I64(x),
            (None, Some(x), _) => RawValue::U64(x),
            (None, None, Some(x)) => RawValue::F64(x),
            _ => unimplemented!("Should handle error here"),
        },
        Value::String(value) => RawValue::String(value),
        Value::Array(values) => RawValue::Array(
            values
                .into_iter()
                .map(|value| json_to_raw_value(super_struct, super_key, value))
                .collect(),
        ),
        Value::Object(values) => {
            let sub_struct_name = format!("{}__{}", super_struct, super_key);
            let values = values
                .into_iter()
                .map(|(key, value)| {
                    let value = json_to_raw_value(&sub_struct_name, &key, value);
                    (key, value)
                })
                .collect();
            RawValue::Struct(RawStructValue {
                struct_name: sub_struct_name,
                fields: values,
            })
        }
    }
}
